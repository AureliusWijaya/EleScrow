import React, {
  useCallback,
  useEffect,
  useRef,
  useState,
  version,
} from "react";
import { IDL, JsonValue } from "@dfinity/candid";
import * as cbor from "cbor-js";
import {
  _SERVICE,
  CanisterInfo,
  Message,
  PostResult,
} from "../../../../declarations/elescrow_backend/elescrow_backend.did";
import { HttpAgent, Actor, ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { sha256 } from "js-sha256";
import { Ed25519KeyIdentity } from "@dfinity/identity";
import { InterfaceFactory } from "@dfinity/candid/lib/esm/idl";
import chatIdlFactory from "./utils/chat-idl-factory";
import chatMessageClass from "./model/chat-message-class";

const CANISTER_ID = "uxrrr-q7777-77774-qaaaq-cai";

const idlFactory: InterfaceFactory = chatIdlFactory;
const messageClass: IDL.VariantClass = chatMessageClass;

const TEST_IDENTITIES: {
  [userKey in string]: { name: string };
} = {
  A: { name: "A" },
  B: { name: "B" },
};

const getIdentity = (userKey: string) => {
  if (!userKey) return null;
  const seed = new TextEncoder().encode(userKey);
  const hashedSeed = sha256.create().update(seed).array();
  return Ed25519KeyIdentity.generate(Uint8Array.from(hashedSeed));
};

function ChatPage(): JSX.Element {
  const [actor, setActor] = useState<ActorSubclass<_SERVICE> | null>(null);
  const [currentUser, setCurrentUser] = useState<string>("");
  const [messages, setMessages] = useState<Array<Message>>([]);
  const [newMessage, setNewMessage] = useState<string>("");
  const [recipient, setRecipient] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>("");
  const [connectionStatus, setConnectionStatus] =
    useState<string>("disconnected");
  const [canisterInfo, setCanisterInfo] = useState<CanisterInfo | null>(null);
  const [isInitializing, setIsInitializing] = useState<boolean>(false);
  const messagesEndRef = useRef<HTMLDivElement | null>(null);
  const wsConnectionRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<number | null>(null);
  const pollingIntervalRef = useRef<number | null>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(scrollToBottom, [messages]);

  const getMessagesHash = (msgs: Array<Message>) => {
    return msgs.map((m) => `${m.id}-${m.read}`).join("|");
  };

  const loadMessages = useCallback(
    async (recipientPrincipal: Principal, isPolling = false) => {
      if (!actor || !recipientPrincipal) return;

      if (!isPolling) {
        setLoading(true);
      }

      try {
        const msgs = await actor.get_conversation_chunk(recipientPrincipal, {
          offset: [],
          limit: [BigInt(100)],
        });

        msgs.sort((a, b) => {
          if (a.timestamp < b.timestamp) return -1;
          if (a.timestamp > b.timestamp) return 1;
          return 0;
        });

        setMessages((prevMsgs) => {
          const prevHash = getMessagesHash(prevMsgs);
          const newHash = getMessagesHash(msgs);

          if (prevHash !== newHash) {
            console.log("Messages updated, refreshing UI.");
            return msgs;
          }
          return prevMsgs;
        });
      } catch (err: any) {
        console.error("Error loading messages:", err);
        if (!isPolling) {
          setError("Error loading messages: " + err.message);
        }
      } finally {
        if (!isPolling) {
          setLoading(false);
        }
      }
    },
    [actor]
  );

  useEffect(() => {
    if (recipient && actor) {
      try {
        const recipientPrincipal = Principal.fromText(recipient);
        loadMessages(recipientPrincipal, false);
      } catch (err) {
        console.warn("Invalid recipient principal:", err);
      }
    } else {
      setMessages([]);
    }
  }, [recipient, actor, loadMessages]);

  useEffect(() => {
    if (pollingIntervalRef.current) {
      clearInterval(pollingIntervalRef.current);
    }

    if (actor && recipient) {
      try {
        const recipientPrincipal = Principal.fromText(recipient);

        pollingIntervalRef.current = setInterval(() => {
          console.log("Polling for new messages...");
          loadMessages(recipientPrincipal, true);
        }, 2000);
      } catch (err) {
        console.warn("Invalid recipient principal for polling:", err);
      }
    }

    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current);
        pollingIntervalRef.current = null;
      }
    };
  }, [actor, recipient, loadMessages]);

  const handleDecodedMessage = useCallback(
    (messageData: any) => {
      const currentIdentity = getIdentity(currentUser);
      if (!currentIdentity) return;
      const currentPrincipalStr = currentIdentity.getPrincipal().toText();

      let recipientPrincipalStr = null;
      try {
        if (recipient)
          recipientPrincipalStr = Principal.fromText(recipient).toString();
      } catch (e) {
        /* ignore invalid principal */
      }

      if ("NewMessage" in messageData) {
        const message: Message = messageData.NewMessage;
        const from = message.from.toString();
        const to = message.to.toString();

        const isRelevant =
          (from === currentPrincipalStr && to === recipientPrincipalStr) ||
          (from === recipientPrincipalStr && to === currentPrincipalStr);

        if (isRelevant) {
          console.log("WebSocket message received, updating UI directly.");
          setMessages((prevMsgs) => {
            if (prevMsgs.some((m) => m.id === message.id)) {
              return prevMsgs;
            }
            const newMsgs = [...prevMsgs, message];
            newMsgs.sort((a, b) => Number(a.timestamp - b.timestamp));
            return newMsgs;
          });
        }
      }

      if ("MessageRead" in messageData) {
        console.log(
          "Message read status updated via WebSocket, updating UI directly."
        );
        const { message_id } = messageData.MessageRead;

        setMessages((prevMsgs) =>
          prevMsgs.map((msg) =>
            msg.id === message_id ? { ...msg, read: true } : msg
          )
        );
      }
    },
    [recipient, currentUser]
  );

  const initWebSocket = useCallback(
    (userKey: string) => {
      if (!userKey || !CANISTER_ID) return;

      if (wsConnectionRef.current) {
        wsConnectionRef.current.close();
      }

      setConnectionStatus("connecting");
      const identity = getIdentity(userKey);
      const currentPrincipal = identity!.getPrincipal().toText();

      const wsUrl = "ws://localhost:8080";
      const ws = new WebSocket(wsUrl);
      wsConnectionRef.current = ws;

      ws.onopen = () => {
        console.log("WebSocket connected");
        setConnectionStatus("connected");
      };

      ws.onmessage = (event) => {
        if (event.data instanceof Blob) {
          event.data.arrayBuffer().then((arrayBuffer) => {
            try {
              const decodedCbor = cbor.decode(arrayBuffer);
              if (decodedCbor && decodedCbor.method_name === "ws_message") {
                const contentBytes = new Uint8Array(decodedCbor.content);
                const decoded = IDL.decode([messageClass], contentBytes);
                handleDecodedMessage(decoded[0]);
              }
            } catch (err) {
              console.error("Failed to decode WebSocket message:", err);
            }
          });
        }
      };

      ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        setConnectionStatus("error");
      };

      ws.onclose = (event) => {
        console.log("WebSocket closed:", event.code, event.reason);
        setConnectionStatus("disconnected");
        if (event.code !== 1000) {
          reconnectTimeoutRef.current = setTimeout(
            () => initWebSocket(userKey),
            3000
          );
        }
      };

      return () => {
        clearTimeout(reconnectTimeoutRef.current!);
        if (
          ws.readyState === WebSocket.OPEN ||
          ws.readyState === WebSocket.CONNECTING
        ) {
          ws.close(1000, "Component unmounting");
        }
      };
    },
    [handleDecodedMessage]
  );

  useEffect(() => {
    if (currentUser && actor) {
      const cleanup = initWebSocket(currentUser);
      return cleanup;
    }
  }, [currentUser, actor, initWebSocket]);

  const createActorForUser = useCallback(async (userKey: string) => {
    setIsInitializing(true);
    setError("");
    try {
      const identity = getIdentity(userKey)!;
      console.log(
        `Created identity for ${userKey}:`,
        identity.getPrincipal().toText()
      );

      const host = "http://127.0.0.1:4943";
      const agent = new HttpAgent({ host, identity });

      if (import.meta.env.DEV) {
        await agent
          .fetchRootKey()
          .catch((err) => console.warn("Could not fetch root key."));
      }

      const newActor: ActorSubclass<_SERVICE> = Actor.createActor(idlFactory, {
        agent,
        canisterId: CANISTER_ID,
      });
      const isHealthy = await newActor.health_check();
      if (!isHealthy) throw new Error("Canister health check failed.");

      const info = await newActor.get_canister_info();
      setActor(newActor);
      setCanisterInfo({
        ...info,
        total_memory_usage: info.total_memory_usage,
      });
    } catch (err: any) {
      console.error("Actor creation failed:", err);
      setError("Failed to connect to canister: " + err.message);
      setActor(null);
    } finally {
      setIsInitializing(false);
    }
  }, []);

  const selectUser = (userKey: string) => {
    setCurrentUser(userKey);
    setMessages([]);
    setRecipient("");
    setError("");
    createActorForUser(userKey);
  };

  const sendMessage = async (e: any) => {
    e.preventDefault();
    if (!actor || !newMessage.trim() || !recipient) return;

    setLoading(true);
    try {
      const recipientPrincipal = Principal.fromText(recipient);
      const result: PostResult = await actor.post_message(
        recipientPrincipal,
        newMessage
      );

      if ("Err" in result) {
        setError("Failed to send message: " + result.Err);
      } else {
        setNewMessage("");
        setTimeout(() => {
          loadMessages(recipientPrincipal, true);
        }, 100);
      }
    } catch (err: any) {
      setError("Error sending message: " + err.message);
    } finally {
      setLoading(false);
    }
  };

  const markAsRead = async (messageId: bigint) => {
    if (!actor || loading) return;
    try {
      const result = await actor.mark_message_read(messageId);
      if ("Ok" in result) {
        if (recipient) {
          const recipientPrincipal = Principal.fromText(recipient);
          setTimeout(() => {
            loadMessages(recipientPrincipal, true);
          }, 100);
        }
      }
    } catch (err: any) {
      setError("Error marking message as read: " + err.message);
    }
  };

  const formatTimestamp = (timestamp: bigint) => {
    return new Date(Number(timestamp / 1000000n)).toLocaleString();
  };

  const getCurrentPrincipal = () => {
    if (!currentUser) return "";
    return getIdentity(currentUser)!.getPrincipal().toText();
  };

  const getOtherUserKey = () => {
    return TEST_IDENTITIES[
      Object.keys(TEST_IDENTITIES).find((x) => x != currentUser)!
    ].name;
  };

  const fillOtherUserPrincipal = () => {
    const otherUserKey = getOtherUserKey();
    const otherIdentity = getIdentity(otherUserKey);
    setRecipient(otherIdentity!.getPrincipal().toText());
  };

  const getConnectionStatusColor = () =>
    ({
      connected: "bg-green-500",
      connecting: "bg-yellow-500 animate-pulse",
      disconnected: "bg-gray-500",
      error: "bg-red-500",
    }[connectionStatus]);

  const getConnectionStatusText = () =>
    ({
      connected: "WebSocket Connected",
      connecting: "Connecting...",
      disconnected: "Disconnected",
      error: "Connection Error",
    }[connectionStatus]);

  useEffect(() => {
    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current);
      }
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
      if (wsConnectionRef.current) {
        wsConnectionRef.current.close();
      }
    };
  }, []);

  if (!currentUser) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center p-4 !text-black">
        <div className="bg-white p-8 rounded-lg shadow-md max-w-md w-full">
          <h1 className="text-2xl font-bold mb-6 text-center">Select User</h1>
          {error && (
            <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
              {error}
            </div>
          )}
          <div className="space-y-3">
            {Object.entries(TEST_IDENTITIES).map(([key, identity]) => (
              <button
                key={key}
                onClick={() => selectUser(key)}
                disabled={isInitializing}
                className="w-full bg-blue-500 hover:bg-blue-600 disabled:bg-blue-300 text-black font-bold py-3 px-4 rounded flex flex-col items-center transition-colors"
              >
                <span className="text-lg">{identity.name}</span>
                <span className="text-xs opacity-75 font-mono break-all px-2">
                  {getIdentity(key)!.getPrincipal().toText()}
                </span>
              </button>
            ))}
          </div>
          {isInitializing && (
            <div className="mt-4 text-center">
              <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
              <p className="text-sm text-gray-600 mt-2">
                Connecting to IC Replica...
              </p>
            </div>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 font-sans !text-black">
      <header className="bg-white shadow-sm border-b sticky top-0 z-10">
        <div className="max-w-4xl mx-auto px-4 py-3 flex justify-between items-center">
          <div>
            <h1 className="text-xl font-bold text-gray-800">
              Chatting as: {TEST_IDENTITIES[currentUser].name}
            </h1>
            {canisterInfo && (
              <p className="text-xs text-gray-500">
                Canister: {canisterInfo.name} v{canisterInfo.version}
              </p>
            )}
          </div>
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">
              <div
                className={`w-3 h-3 rounded-full ${getConnectionStatusColor()}`}
              ></div>
              <span className="text-sm text-gray-600">
                {getConnectionStatusText()}
              </span>
            </div>
            <button
              onClick={() => setCurrentUser("")}
              className="bg-red-500 hover:bg-red-600 text-black px-4 py-2 rounded text-sm transition-colors"
            >
              Switch User
            </button>
          </div>
        </div>
      </header>

      <main className="max-w-4xl mx-auto p-4">
        {error && (
          <div
            className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4"
            role="alert"
          >
            {error}
          </div>
        )}
        <div className="bg-white rounded-lg shadow-sm p-4 mb-4">
          <div className="flex flex-col sm:flex-row sm:space-x-2">
            <input
              type="text"
              value={recipient}
              onChange={(e) => setRecipient(e.target.value)}
              placeholder="Enter recipient's principal ID..."
              className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 font-mono text-sm mb-2 sm:mb-0"
            />
            <div className="flex space-x-2">
              <button
                onClick={fillOtherUserPrincipal}
                className="flex-1 sm:flex-none bg-green-500 hover:bg-green-600 text-black px-4 py-2 rounded text-sm transition-colors"
              >
                Chat with {TEST_IDENTITIES[getOtherUserKey()].name}
              </button>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm">
          <div className="h-96 overflow-y-auto p-4 space-y-4">
            {messages.length > 0 ? (
              messages.map((message) => {
                const isOwnMessage =
                  message.from.toString() === getCurrentPrincipal();
                return (
                  <div
                    key={message.id.toString()}
                    className={`flex ${
                      isOwnMessage ? "justify-end" : "justify-start"
                    }`}
                  >
                    <div
                      className={`max-w-xs lg:max-w-md px-4 py-2 rounded-lg cursor-pointer ${
                        isOwnMessage
                          ? "bg-blue-500 text-black"
                          : "bg-gray-200 text-gray-800"
                      }`}
                      onClick={() =>
                        !isOwnMessage && !message.read && markAsRead(message.id)
                      }
                    >
                      <p className="text-sm break-words">{message.text}</p>
                      <div className="flex items-center justify-between mt-1 text-xs opacity-75">
                        <span>{formatTimestamp(message.timestamp)}</span>
                        {isOwnMessage && (
                          <span className={message.read ? "text-blue-200" : ""}>
                            {message.read ? "✓✓" : "✓"}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>
                );
              })
            ) : (
              <div className="text-center text-gray-500 py-8">
                {loading
                  ? "Loading..."
                  : recipient
                  ? "No messages yet. Start the conversation!"
                  : "Select a chat partner to begin messaging."}
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>
        </div>

        <form
          onSubmit={sendMessage}
          className="bg-white rounded-lg shadow-sm p-4 mt-4 flex space-x-2"
        >
          <input
            type="text"
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            placeholder="Type a message..."
            className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={!recipient || loading}
          />
          <button
            type="submit"
            disabled={!newMessage.trim() || !recipient || loading}
            className="bg-blue-500 hover:bg-blue-600 disabled:bg-blue-300 text-black px-6 py-2 rounded transition-colors"
          >
            Send
          </button>
        </form>
      </main>
    </div>
  );
}

export default ChatPage;
