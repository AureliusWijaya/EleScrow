import { InterfaceFactory } from "@dfinity/candid/lib/esm/idl";

const idlFactory: InterfaceFactory = ({ IDL }) => {
    const PostResult = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
    const PaginationParams = IDL.Record({
        offset: IDL.Opt(IDL.Nat64),
        limit: IDL.Opt(IDL.Nat64),
    });
    const Message = IDL.Record({
        id: IDL.Nat64,
        to: IDL.Principal,
        text: IDL.Text,
        read: IDL.Bool,
        from: IDL.Principal,
        timestamp: IDL.Nat64,
    });

    const CanisterInfo = IDL.Record({
        name: IDL.Text,
        version: IDL.Text,
        modules: IDL.Vec(IDL.Text),
        total_memory_usage: IDL.Nat64,
    });
    return IDL.Service({
        get_canister_info: IDL.Func([], [CanisterInfo], ["query"]),
        get_conversation_chunk: IDL.Func(
            [IDL.Principal, PaginationParams],
            [IDL.Vec(Message)],
            ["query"]
        ),
        get_message_count: IDL.Func([], [IDL.Nat64], ["query"]),
        health_check: IDL.Func([], [IDL.Bool], ["query"]),
        mark_message_read: IDL.Func([IDL.Nat64], [PostResult], []),
        post_message: IDL.Func([IDL.Principal, IDL.Text], [PostResult], []),
    });
};

export default idlFactory;
