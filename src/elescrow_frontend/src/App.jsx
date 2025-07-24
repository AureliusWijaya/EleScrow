import React, { useState, useEffect } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { AuthClient } from "@dfinity/auth-client";

const backendCanisterId = "BACKEND_CANISTER_ID";
const localLedgerCanisterId = "LEDGER_CANISTER_ID";
const localInternetIdentity = "INTERNET_IDENTITY_ID";

const LOCAL_HOST = "http://127.0.0.1:4943";
const identityProvider = `http://127.0.0.1:4943/?canisterId=${localInternetIdentity}`;

const icrcIdlFactory = ({ IDL }) => {
  const Account = IDL.Record({ owner: IDL.Principal, subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)) });
  const TransferArgs = IDL.Record({ to: Account, fee: IDL.Opt(IDL.Nat), memo: IDL.Opt(IDL.Vec(IDL.Nat8)), from_subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)), created_at_time: IDL.Opt(IDL.Nat64), amount: IDL.Nat, });
  const TransferResult = IDL.Variant({ Ok: IDL.Nat, Err: IDL.Variant({ GenericError: IDL.Record({ message: IDL.Text, error_code: IDL.Nat }), TemporarilyUnavailable: IDL.Null, BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }), Duplicate: IDL.Record({ duplicate_of: IDL.Nat }), BadFee: IDL.Record({ expected_fee: IDL.Nat }), CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }), TooOld: IDL.Null, InsufficientFunds: IDL.Record({ balance: IDL.Nat }), }), });
  return IDL.Service({ 'icrc1_transfer': IDL.Func([TransferArgs], [TransferResult], []), });
};
const idlFactory = ({ IDL }) => {
  const ApiError = IDL.Variant({ 'Unauthorized': IDL.Record({ 'reason': IDL.Text }), 'InvalidState': IDL.Record({ 'current_state': IDL.Text, 'required_state': IDL.Text }), 'NotFound': IDL.Record({ 'resource': IDL.Text }), 'InsufficientFunds': IDL.Record({ 'available': IDL.Nat64, 'required': IDL.Nat64 }), });
  const DisputeResolution = IDL.Variant({ 'ReleaseToRecipient': IDL.Null, 'RefundToSender': IDL.Null, 'SplitBetweenParties': IDL.Record({ 'sender_percentage': IDL.Nat8 }), });
  const TransactionStatus = IDL.Variant({ 'Pending': IDL.Null, 'InEscrow': IDL.Null, 'SubmittedForReview': IDL.Record({ 'submitted_at': IDL.Nat64 }), 'Completed': IDL.Null, 'Cancelled': IDL.Record({ 'reason': IDL.Text, 'cancelled_by': IDL.Principal, 'cancelled_at': IDL.Nat64 }), 'Disputed': IDL.Record({ 'reason': IDL.Text, 'disputed_by': IDL.Principal, 'disputed_at': IDL.Nat64 }), 'Resolved': IDL.Record({ 'resolution': DisputeResolution, 'resolved_at': IDL.Nat64, 'resolved_by': IDL.Principal }), });
  const Transaction = IDL.Record({ 'id': IDL.Nat64, 'to': IDL.Principal, 'from': IDL.Principal, 'amount': IDL.Nat64, 'status': TransactionStatus, 'description': IDL.Text, });
  const Result_Transaction = IDL.Variant({ 'Ok': Transaction, 'Err': ApiError });
  const CreateTransactionRequest = IDL.Record({ 'to': IDL.Principal, 'amount': IDL.Nat64, 'description': IDL.Text, 'transaction_type': IDL.Variant({ 'Escrow': IDL.Record({ 'release_conditions': IDL.Vec(IDL.Text), 'auto_release_after': IDL.Opt(IDL.Nat64) }) }), 'currency': IDL.Variant({ 'ICP': IDL.Null }), 'escrow_agent': IDL.Opt(IDL.Principal), 'deadline': IDL.Opt(IDL.Nat64), 'category': IDL.Opt(IDL.Variant({ 'Business': IDL.Null })), 'tags': IDL.Vec(IDL.Text), });
  const Balance = IDL.Record({ 'available': IDL.Nat64, 'locked': IDL.Nat64 });
  const Result_Balance = IDL.Variant({ 'Ok': Balance, 'Err': ApiError });
  return IDL.Service({ 'create_transaction': IDL.Func([CreateTransactionRequest], [Result_Transaction], []), 'accept_escrow_terms': IDL.Func([IDL.Nat64], [Result_Transaction], []), 'submit_escrow_work': IDL.Func([IDL.Nat64], [Result_Transaction], []), 'complete_transaction': IDL.Func([IDL.Nat64], [Result_Transaction], []), 'raise_dispute': IDL.Func([IDL.Nat64, IDL.Text], [Result_Transaction], []), 'get_transaction': IDL.Func([IDL.Nat64], [Result_Transaction], ['query']), 'get_balance': IDL.Func([], [Result_Balance], ['query']), 'deposit': IDL.Func([IDL.Nat64], [IDL.Variant({ 'Ok': IDL.Nat64, 'Err': ApiError })], []), });
};


function App() {
  const [authClient, setAuthClient] = useState(null);
  const [identity, setIdentity] = useState(null);
  const [principal, setPrincipal] = useState(null);
  const [backendActor, setBackendActor] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [balance, setBalance] = useState(null);
  const [depositAmount, setDepositAmount] = useState('');
  const [escrowAmount, setEscrowAmount] = useState('');
  const [escrowRecipient, setEscrowRecipient] = useState('');

  useEffect(() => {
    AuthClient.create().then(async (client) => {
      setAuthClient(client);
      if (await client.isAuthenticated()) {
        handleAuthenticated(client);
      }
    });
  }, []);

  const handleLogin = async () => {
    if (!authClient) return;
    setIsLoading(true);
    await authClient.login({
      identityProvider,
      onSuccess: () => handleAuthenticated(authClient),
      onError: (err) => alert(`Login failed: ${err}`),
    });
    setIsLoading(false);
  };

  const handleAuthenticated = async (client) => {
    const newIdentity = client.getIdentity();
    const newPrincipal = newIdentity.getPrincipal();
    const agent = new HttpAgent({ identity: newIdentity, host: LOCAL_HOST });
    await agent.fetchRootKey();
    const actor = Actor.createActor(idlFactory, { agent, canisterId: backendCanisterId });
    setIdentity(newIdentity);
    setPrincipal(newPrincipal);
    setAuthClient(client);
    setBackendActor(actor);
    await fetchBalance(actor);
  };

  const handleLogout = async () => {
    if (!authClient) return;
    await authClient.logout();
    setIdentity(null);
    setPrincipal(null);
    setBackendActor(null);
    setBalance(null);
  };

  const fetchBalance = async (actor) => {
    const actorToUse = actor || backendActor;
    if (!actorToUse) return;
    try {
      const res = await actorToUse.get_balance();
      if (res.Ok) setBalance(res.Ok);
      else console.error('Balance fetch error:', res.Err);
    } catch (e) { console.error("Failed to fetch balance:", e); }
  };
  
  const handleDeposit = async () => {
    if (!identity || !depositAmount) {
      alert("Please login and enter an amount to deposit.");
      return;
    }
    const amount = BigInt(depositAmount);
    if (amount <= 10000n) {
      alert("Deposit amount must be greater than the transaction fee.");
      return;
    }
    setIsLoading(true);
    try {
      const agent = new HttpAgent({ identity, host: LOCAL_HOST });
      await agent.fetchRootKey();
      const ledgerActor = Actor.createActor(icrcIdlFactory, { agent, canisterId: localLedgerCanisterId });
      const transferResult = await ledgerActor.icrc1_transfer({
        to: { owner: Principal.fromText(backendCanisterId), subaccount: [] },
        amount: amount,
        fee: [], memo: [], from_subaccount: [], created_at_time: [],
      });
      if (transferResult.Err) throw new Error(`Ledger transfer failed: ${Object.keys(transferResult.Err)[0]}`);
      const result = await backendActor.deposit(amount);
      if (result.Ok) {
        alert(`Deposit of ${Number(amount) / 1e8} LICP successful!`);
        await fetchBalance(backendActor);
        setDepositAmount('');
      } else { throw new Error(`Backend canister error: ${Object.keys(result.Err)[0]}`); }
    } catch (error) {
      alert(`Error during deposit: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateEscrow = async (e) => {
    e.preventDefault();
    if (!escrowRecipient || !escrowAmount) {
        alert("Please provide a recipient and an amount.");
        return;
    }
    let recipient;
    try {
      recipient = Principal.fromText(escrowRecipient);
    } catch (error) { alert("Invalid recipient principal format."); return; }
    const amount = BigInt(escrowAmount);
    setIsLoading(true);
    try {
      const request = { to: recipient, amount, description: "An escrow created via Internet Identity", transaction_type: { 'Escrow': { release_conditions: [], auto_release_after: [] } }, currency: { 'ICP': null }, escrow_agent: [], deadline: [], category: [], tags: [], };
      const result = await backendActor.create_transaction(request);
      if (result.Ok) {
        await fetchBalance(backendActor);
        alert(`Transaction created with ID: ${result.Ok.id}`);
        setEscrowAmount('');
        setEscrowRecipient('');
      } else { throw new Error(`Canister error: ${Object.keys(result.Err)[0]}`); }
    } catch (error) { alert(`Error: ${error.message}`); } finally { setIsLoading(false); }
  };

  if (!principal) {
    return (
      <main style={{ padding: '20px', textAlign: 'center' }}>
        <h1>Escrow dApp</h1>
        <button onClick={handleLogin} disabled={isLoading || !authClient}>
          {isLoading ? 'Logging in...' : 'Login with Internet Identity'}
        </button>
      </main>
    );
  }

  return (
    <main style={{ padding: '20px', fontFamily: 'sans-serif' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h1>Escrow dApp</h1>
        <button onClick={handleLogout}>Logout</button>
      </div>
      <p>Welcome: <strong>{principal.toText()}</strong></p>
      {balance && (
        <p>Internal Balance: <strong>{Number(balance.available) / 1e8} LICP</strong> (Locked: {Number(balance.locked) / 1e8} LICP)</p>
      )}
      <hr />
      <div>
        <h2>1. Deposit Funds into Smart Contract</h2>
        <input name="amount" type="number" placeholder="Amount in e8s (1 LICP = 100,000,000)" value={depositAmount} onChange={(e) => setDepositAmount(e.target.value)} required style={{ marginRight: '10px', padding: '5px' }} />
        <button onClick={handleDeposit} disabled={isLoading}>Deposit LICP</button>
      </div>
      <hr />
      <div>
        <h2>2. Create New Escrow Transaction</h2>
        <form onSubmit={handleCreateEscrow} style={{ margin: '20px 0' }}>
            <input name="recipient" type="text" placeholder="Recipient Principal ID" value={escrowRecipient} onChange={(e) => setEscrowRecipient(e.target.value)} required style={{ width: '500px', display: 'block', marginBottom: '10px', padding: '5px' }} />
            <input name="amount" type="number" placeholder="Amount in e8s (1 LICP = 100,000,000)" value={escrowAmount} onChange={(e) => setEscrowAmount(e.target.value)} required style={{ display: 'block', marginBottom: '10px', padding: '5px' }} />
            <button type="submit" disabled={isLoading}>Create Escrow</button>
        </form>
      </div>
    </main>
  );
}

export default App;