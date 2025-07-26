import { IDL } from "@dfinity/candid";

const chatMessageClass = IDL.Variant({
  NewMessage: IDL.Record({
    id: IDL.Nat64,
    from: IDL.Principal,
    to: IDL.Principal,
    text: IDL.Text,
    timestamp: IDL.Nat64,
    read: IDL.Bool,
  }),
  MessageRead: IDL.Record({
    message_id: IDL.Nat64,
  }),
});

export default chatMessageClass;
