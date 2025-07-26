import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import Button from "../../shared/components/Button";
import Input from "../../shared/components/Input";
import Select from "../../shared/components/Select";
import Textarea from "../../shared/components/Textarea";
import { notifications } from "@mantine/notifications";
import { icpService } from "../../shared/services/icp-service";
import { useLoggedInUserPrincipal, useUserStoreActions } from "../../shared/store/user-store";
import { Principal } from "@dfinity/principal";
import { useAuth } from "../../shared/hooks/useAuth";

function CreateTransactionPage(): JSX.Element {
  const navigate = useNavigate();
  const { setLoggedInUserPrincipal } = useUserStoreActions();
  const { isAuthenticated, loggedInPrincipal } = useAuth();
  
  const [username, setUsername] = useState("");
  const [recipientPrincipal, setRecipientPrincipal] = useState<Principal | null>(null);
  const [amount, setAmount] = useState("");
  const [currency, setCurrency] = useState("USDT");
  const [description, setDescription] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [isSearching, setIsSearching] = useState(false);

  useEffect(() => {
    icpService.initialize();
  }, []);

  const handleSearchUser = async () => {
    if (!username.trim()) {
      notifications.show({
        title: "Username Required",
        message: "Please enter a username to search",
        color: "red",
        icon: "⚠️",
      });
      return;
    }
    
    setIsSearching(true);
    try {
      const result = await icpService.getUserByUsername(username);
      
      if (result.Ok) {
        setRecipientPrincipal(result.Ok.principal);
        notifications.show({
          title: "User Found! ✅",
          message: `Found user: ${result.Ok.display_name || result.Ok.username}`,
          color: "green",
          icon: "👤",
        });
      } else {
        setRecipientPrincipal(null);
        notifications.show({
          title: "User Not Found",
          message: result.Err || "No user found with that username",
          color: "red",
          icon: "❌",
        });
      }
    } catch (error: any) {
      console.error("Search error:", error);
    notifications.show({
        title: "Search Failed",
        message: error.message || "Failed to search for user",
        color: "red",
        icon: "⚠️",
      });
    } finally {
      setIsSearching(false);
    }
  };
  

  const handleCreateTransaction = async () => {
    if (!loggedInPrincipal) {
      notifications.show({
        title: "Authentication Required",
        message: "Please connect your wallet first",
        color: "red",
        icon: "🔐",
      });
      return;
    }

    if (!recipientPrincipal || !amount || !description) {
      notifications.show({
        title: "Missing Information",
        message: "Please fill in all fields and search for a valid user",
        color: "red",
        icon: "⚠️",
      });
      return;
    }

    const amountValue = parseFloat(amount);
    if (amountValue <= 0) {
      notifications.show({
        title: "Invalid Amount",
        message: "Amount must be greater than 0",
        color: "red",
        icon: "💰",
      });
      return;
    }

    setIsLoading(true);
    
    try {
      let amountInSmallestUnits: bigint;
      switch (currency) {
        case "ICP":
          amountInSmallestUnits = BigInt(Math.floor(amountValue * 100000000));
          break;
        case "USDT":
          amountInSmallestUnits = BigInt(Math.floor(amountValue * 1000000));
          break;
        default:
          amountInSmallestUnits = BigInt(Math.floor(amountValue * 100000000));
      }

      const request = {
        to: recipientPrincipal,
        transaction_type: { DirectPayment: null },
        amount: amountInSmallestUnits,
        currency: currency === "ICP" ? { ICP: null } : { USDT: null },
        description: description,
        escrow_agent: undefined,
        deadline: undefined,
        category: { Business: null },
        tags: ["web3", "elescrow"]
      };

      console.log("Creating transaction:", request);

      const result = await icpService.createTransaction(request);
      
      if (result.Ok) {
      notifications.show({
        title: "Transaction Created Successfully! 🎉",
        message: `${amount} ${currency} transaction created for ${username}`,
        color: "green",
        icon: "✅",
        autoClose: 5000,
      });

      setUsername("");
        setRecipientPrincipal(null);
      setAmount("");
      setDescription("");
        
        setTimeout(() => {
          navigate("/transactions");
        }, 2000);
      } else {
        throw new Error(result.Err || "Unknown error");
      }
    } catch (error: any) {
      console.error("Transaction creation failed:", error);
      
      let errorMessage = "Something went wrong. Please try again.";
      
      if (error.message?.includes("InsufficientFunds")) {
        errorMessage = "Insufficient funds in your wallet";
      } else if (error.message?.includes("ValidationError")) {
        errorMessage = "Invalid transaction data";
      } else if (error.message?.includes("SystemPaused")) {
        errorMessage = "System is temporarily paused";
      }
      
      notifications.show({
        title: "Transaction Failed",
        message: errorMessage,
        color: "red",
        icon: "❌",
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="w-full min-h-screen bg-primary flex items-center justify-center py-10">
      <div className="w-full max-w-md mx-auto px-6">
        <div className="flex flex-col gap-8">
          <h1 className="text-4xl font-semibold text-primary-text text-center">
            Create a Transaction
          </h1>

          <div className="flex flex-col gap-6">
            <div className="relative">
              <Input
                label="Search for User"
                type="text"
                placeholder="Username"
                value={username}
                onChange={setUsername}
              />
              <button
                onClick={handleSearchUser}
                disabled={isSearching}
                className={`absolute right-3 bottom-3 text-secondary-text hover:text-secondary transition-colors ${
                  isSearching ? 'opacity-50 cursor-not-allowed' : ''
                }`}
              >
                {isSearching ? (
                  <div className="w-4 h-4 border-2 border-secondary-text border-t-transparent rounded-full animate-spin"></div>
                ) : (
                <i className="bi bi-search text-lg"></i>
                )}
              </button>
            </div>

              {recipientPrincipal && (
                <div className="bg-green-900/20 border border-green-500/30 rounded-lg p-3">
                  <div className="flex items-center gap-2 text-green-400">
                    <i className="bi bi-check-circle"></i>
                    <span className="text-sm">User found: {username}</span>
                  </div>
                  <div className="text-xs text-green-300 mt-1 font-mono">
                    {recipientPrincipal.toText()}
                  </div>
                </div>
              )}

            <div className="flex gap-3">
              <Select
                label="Currency"
                value={currency}
                onChange={setCurrency}
                className="min-w-[120px]"
              >
                <option value="USDT" className="bg-primary">USDT</option>
                <option value="BTC" className="bg-primary">BTC</option>
                <option value="ETH" className="bg-primary">ETH</option>
                <option value="ICP" className="bg-primary">ICP</option>
              </Select>
              
              <Input
                label="Amount"
                type="number"
                placeholder="Amount"
                value={amount}
                onChange={setAmount}
                className="flex-1"
              />
            </div>

            <Textarea
              label="Description"
              placeholder="Description"
              value={description}
              onChange={setDescription}
              rows={6}
            />
          </div>

          <Button 
              className={`w-full !py-4 !text-base !font-semibold ${
                isLoading || !recipientPrincipal ? 'opacity-50 cursor-not-allowed' : ''
              }`}
              click={isLoading || !recipientPrincipal ? undefined : handleCreateTransaction}
          >
            {isLoading ? (
              <div className="flex items-center justify-center gap-2">
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                Creating...
              </div>
            ) : (
                'Create Transaction'
            )}
          </Button>
        </div>
      </div>
    </div>
  );
}

export default CreateTransactionPage; 