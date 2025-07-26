import React, { useState, useEffect, useCallback } from 'react';
import { Actor, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { AuthClient } from "@dfinity/auth-client";

const backendCanisterId = "PASTE_YOUR_BACKEND_CANISTER_ID";
const localLedgerCanisterId = "PASTE_YOUR_LOCAL_LEDGER_CANISTER_ID";
const internetIdentityCanisterId = "PASTE_YOUR_LOCAL_INTERNET_IDENTITY_CANISTER_ID";

const localHost = "http://localhost:4943";
const identityProvider = `http://${internetIdentityCanisterId}.localhost:4943`;

const icrcIdlFactory = ({ IDL }) => {
  const Account = IDL.Record({ owner: IDL.Principal, subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)) });
  const TransferArgs = IDL.Record({ to: Account, fee: IDL.Opt(IDL.Nat), memo: IDL.Opt(IDL.Vec(IDL.Nat8)), from_subaccount: IDL.Opt(IDL.Vec(IDL.Nat8)), created_at_time: IDL.Opt(IDL.Nat64), amount: IDL.Nat, });
  const TransferResult = IDL.Variant({ Ok: IDL.Nat, Err: IDL.Variant({ GenericError: IDL.Record({ message: IDL.Text, error_code: IDL.Nat }), TemporarilyUnavailable: IDL.Null, BadBurn: IDL.Record({ min_burn_amount: IDL.Nat }), Duplicate: IDL.Record({ duplicate_of: IDL.Nat }), BadFee: IDL.Record({ expected_fee: IDL.Nat }), CreatedInFuture: IDL.Record({ ledger_time: IDL.Nat64 }), TooOld: IDL.Null, InsufficientFunds: IDL.Record({ balance: IDL.Nat }), }), });
  return IDL.Service({ 'icrc1_transfer': IDL.Func([TransferArgs], [TransferResult], []), });
};


const idlFactory = ({ IDL }) => {
  const ApiError = IDL.Variant({
    'NotFound': IDL.Record({ 'resource': IDL.Text }),
    'ValidationError': IDL.Record({ 'field': IDL.Text, 'message': IDL.Text }),
    'Unauthorized': IDL.Record({ 'reason': IDL.Text }),
    'InvalidState': IDL.Record({ 'current_state': IDL.Text, 'required_state': IDL.Text }),
    'InsufficientFunds': IDL.Record({ 'available': IDL.Nat64, 'required': IDL.Nat64 }),
  });

  const DisputeResolution = IDL.Variant({
    'ReleaseToRecipient': IDL.Null,
    'RefundToSender': IDL.Null,
    'SplitBetweenParties': IDL.Record({ 'sender_percentage': IDL.Nat8 }),
  });

  const TransactionStatus = IDL.Variant({
    'Pending': IDL.Null,
    'InEscrow': IDL.Null,
    'SubmittedForReview': IDL.Record({ 'submitted_at': IDL.Nat64 }),
    'Completed': IDL.Null,
    'Cancelled': IDL.Record({ 'reason': IDL.Text, 'cancelled_by': IDL.Principal, 'cancelled_at': IDL.Nat64 }),
    'Disputed': IDL.Record({ 'reason': IDL.Text, 'disputed_by': IDL.Principal, 'disputed_at': IDL.Nat64 }),
    'Resolved': IDL.Record({ 'resolution': DisputeResolution, 'resolved_at': IDL.Nat64, 'resolved_by': IDL.Principal }),
    'Failed': IDL.Record({ 'failed_at': IDL.Nat64, 'reason': IDL.Text }),
    'UnderReview': IDL.Record({ 'review_started_at': IDL.Nat64, 'reviewer': IDL.Principal }),
    'Refunded': IDL.Record({ 'refund_transaction_id': IDL.Nat64, 'refunded_at': IDL.Nat64 }),
    'Approved': IDL.Null,
    'Draft': IDL.Null,
    'Processing': IDL.Null,
    'PartiallyRefunded': IDL.Record({ 'refund_transaction_ids': IDL.Vec(IDL.Nat64), 'refunded_amount': IDL.Nat64 }),
  });

  const PaymentFrequency = IDL.Variant({
    'BiWeekly': IDL.Null, 'OneTime': IDL.Null, 'Weekly': IDL.Null, 'Quarterly': IDL.Null, 'Daily': IDL.Null,
    'Custom': IDL.Record({ 'interval_days': IDL.Nat32 }), 'Monthly': IDL.Null, 'Yearly': IDL.Null,
  });

  const PaymentSchedule = IDL.Record({
    'amount_per_payment': IDL.Nat64, 'payments_completed': IDL.Nat32, 'end_date': IDL.Opt(IDL.Nat64),
    'start_date': IDL.Nat64, 'total_payments': IDL.Opt(IDL.Nat32), 'frequency': PaymentFrequency, 'next_payment_date': IDL.Nat64,
  });

  const TransactionType = IDL.Variant({
    'Escrow': IDL.Record({ 'release_conditions': IDL.Vec(IDL.Text), 'auto_release_after': IDL.Opt(IDL.Nat64) }),
    'Release': IDL.Null, 'Deposit': IDL.Null, 'Refund': IDL.Record({ 'original_transaction_id': IDL.Nat64 }),
    'Reversal': IDL.Null, 'Dispute': IDL.Record({ 'evidence': IDL.Vec(IDL.Text), 'reason': IDL.Text }),
    'Withdrawal': IDL.Null, 'ScheduledPayment': IDL.Record({ 'schedule': PaymentSchedule }), 'DirectPayment': IDL.Null,
  });

  const TransactionNote = IDL.Record({ 'content': IDL.Text, 'is_private': IDL.Bool, 'created_at': IDL.Nat64, 'author': IDL.Principal });
  const Attachment = IDL.Record({ 'id': IDL.Text, 'url': IDL.Text, 'name': IDL.Text, 'size': IDL.Nat64, 'mime_type': IDL.Text, 'uploaded_at': IDL.Nat64, 'uploaded_by': IDL.Principal });
  const TransactionCategory = IDL.Variant({ 'Refund': IDL.Null, 'Investment': IDL.Null, 'Salary': IDL.Null, 'Personal': IDL.Null, 'Freelance': IDL.Null, 'Business': IDL.Null, 'Other': IDL.Record({ 'name': IDL.Text }) });
  
  const TransactionMetadata = IDL.Record({
    'invoice_id': IDL.Opt(IDL.Text), 'tags': IDL.Vec(IDL.Text), 'notes': IDL.Vec(TransactionNote),
    'category': IDL.Opt(TransactionCategory), 'order_id': IDL.Opt(IDL.Text),
    'custom_fields': IDL.Vec(IDL.Record({ '0': IDL.Text, '1': IDL.Text })), 'attachments': IDL.Vec(Attachment),
  });

  const Currency = IDL.Variant({ 'ICP': IDL.Null, 'USDT': IDL.Null, 'Custom': IDL.Record({ 'decimals': IDL.Nat8, 'symbol': IDL.Text }), 'Cycles': IDL.Null });

  const Transaction = IDL.Record({
    'id': IDL.Nat64, 'to': IDL.Principal, 'fee': IDL.Nat64, 'status': TransactionStatus, 'updated_at': IDL.Nat64,
    'transaction_type': TransactionType, 'metadata': TransactionMetadata, 'from': IDL.Principal, 'escrow_agent': IDL.Opt(IDL.Principal),
    'description': IDL.Text, 'deadline': IDL.Opt(IDL.Nat64), 'created_at': IDL.Nat64, 'currency': Currency, 'completed_at': IDL.Opt(IDL.Nat64), 'amount': IDL.Nat64,
  });

  const Result_Transaction = IDL.Variant({ 'Ok': Transaction, 'Err': ApiError });
  const CreateTransactionRequest = IDL.Record({ 'to': IDL.Principal, 'amount': IDL.Nat64, 'description': IDL.Text, 'transaction_type': TransactionType, 'currency': Currency, 'escrow_agent': IDL.Opt(IDL.Principal), 'deadline': IDL.Opt(IDL.Nat64), 'category': IDL.Opt(TransactionCategory), 'tags': IDL.Vec(IDL.Text) });
  const Balance = IDL.Record({ 'available': IDL.Nat64, 'locked': IDL.Nat64 });
  const Result_Balance = IDL.Variant({ 'Ok': Balance, 'Err': ApiError });
  
  const TimeFilter = IDL.Record({ 'end': IDL.Opt(IDL.Nat64), 'start': IDL.Opt(IDL.Nat64) });
  const TransactionFilter = IDL.Record({ 'to': IDL.Opt(IDL.Principal), 'status': IDL.Opt(IDL.Vec(TransactionStatus)), 'transaction_type': IDL.Opt(IDL.Vec(TransactionType)), 'min_amount': IDL.Opt(IDL.Nat64), 'date_range': IDL.Opt(TimeFilter), 'from': IDL.Opt(IDL.Principal), 'tags': IDL.Opt(IDL.Vec(IDL.Text)), 'currency': IDL.Opt(Currency), 'category': IDL.Opt(TransactionCategory), 'max_amount': IDL.Opt(IDL.Nat64) });
  const PaginationParams = IDL.Record({ 'offset': IDL.Nat64, 'limit': IDL.Nat64 });
  const Result_VecTransaction = IDL.Variant({ 'Ok': IDL.Vec(Transaction), 'Err': ApiError });

  return IDL.Service({
    'create_transaction': IDL.Func([CreateTransactionRequest], [Result_Transaction], []),
    'accept_escrow_terms': IDL.Func([IDL.Nat64], [Result_Transaction], []),
    'submit_escrow_work': IDL.Func([IDL.Nat64], [Result_Transaction], []),
    'complete_transaction': IDL.Func([IDL.Nat64], [Result_Transaction], []),
    'raise_dispute': IDL.Func([IDL.Nat64, IDL.Text], [Result_Transaction], []),
    'get_transaction': IDL.Func([IDL.Nat64], [Result_Transaction], ['query']),
    'get_balance': IDL.Func([], [Result_Balance], ['query']),
    'deposit': IDL.Func([IDL.Nat64], [IDL.Variant({ 'Ok': IDL.Nat64, 'Err': ApiError })], []),
    'get_my_transactions': IDL.Func([IDL.Opt(TransactionFilter), PaginationParams], [Result_VecTransaction], ['query']),
  });
};

const createLocalAgent = async (identity) => {
  const agent = new HttpAgent({ 
    identity,
    host: localHost,
    verifyQuerySignatures: false,
  });
 
  await agent.fetchRootKey();
 
  return agent;
};

const jsonReplacer = (key, value) =>
  typeof value === 'bigint' ? value.toString() : value;

function App() {
  const [authClient, setAuthClient] = useState(null);
  const [identity, setIdentity] = useState(null);
  const [principal, setPrincipal] = useState(null);
  const [backendActor, setBackendActor] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  const [balance, setBalance] = useState(null);
  const [transactions, setTransactions] = useState([]);
  const [depositAmount, setDepositAmount] = useState('');
  const [escrowAmount, setEscrowAmount] = useState('');
  const [escrowRecipient, setEscrowRecipient] = useState('');
  const [error, setError] = useState('');

  const fetchAllData = useCallback(async (actor) => {
    if (!actor) return;
    setIsLoading(true);
    try {
      const [balanceRes, transactionsRes] = await Promise.all([
        actor.get_balance(),
        actor.get_my_transactions([], { offset: 0n, limit: 100n })
      ]);

      if ('Ok' in balanceRes) {
        setBalance(balanceRes.Ok);
      } else {
        console.error('Balance fetch error:', JSON.stringify(balanceRes.Err, jsonReplacer));
        setError(`Failed to fetch balance: ${JSON.stringify(balanceRes.Err, jsonReplacer)}`);
      }
      
      if ('Ok' in transactionsRes) {
        setTransactions(transactionsRes.Ok.sort((a, b) => Number(b.id - a.id)));
      } else {
        console.error('Transactions fetch error:', JSON.stringify(transactionsRes.Err, jsonReplacer));
        setError(`Failed to fetch transactions: ${JSON.stringify(transactionsRes.Err, jsonReplacer)}`);
      }

    } catch (e) { 
      console.error("Failed to fetch data:", e);
      setError("Failed to fetch data from the canister. Check your connection and canister status.");
    } finally {
      setIsLoading(false);
    }
  }, []);

  const handleAuthenticated = useCallback(async (client) => {
    try {
      const newIdentity = client.getIdentity();
      const newPrincipal = newIdentity.getPrincipal();
      
      console.log("Logged in with LOCAL identity, principal:", newPrincipal.toText());

      const agent = await createLocalAgent(newIdentity);
      const actor = Actor.createActor(idlFactory, { agent, canisterId: backendCanisterId });
      
      setIdentity(newIdentity);
      setPrincipal(newPrincipal);
      setAuthClient(client);
      setBackendActor(actor);
      
      setError('');
      
      await fetchAllData(actor);
    } catch (error) {
      console.error("Authentication handling failed:", error);
      setError(`Authentication failed: ${error.message}`);
      setIsLoading(false);
    }
  }, [fetchAllData]);

  useEffect(() => {
    AuthClient.create({
      identityProvider: {
        derivationOrigin: `http://${internetIdentityCanisterId}.localhost:4943`
      },
      idleOptions: {
        disableIdle: true,
      }
    }).then(async (client) => {
      setAuthClient(client);
      if (await client.isAuthenticated()) {
        await handleAuthenticated(client);
      } else {
        setIsLoading(false);
      }
    }).catch(err => {
      console.error("AuthClient creation failed:", err);
      setError("Could not create authentication client.");
      setIsLoading(false);
    });
  }, [handleAuthenticated]);

  const handleLogin = async () => {
    if (!authClient) return;
    setIsLoading(true);
    setError('');
    
    try {
      await authClient.login({
        identityProvider,
        maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000),
        onSuccess: () => handleAuthenticated(authClient),
        onError: (err) => {
          console.error("Login error:", err);
          setError(`Login failed: ${err}. Make sure your local replica and II canister are running.`);
          setIsLoading(false);
        },
      });
    } catch (error) {
      console.error("Login process failed:", error);
      setError(`Login process failed: ${error.message}`);
      setIsLoading(false);
    }
  };

  const handleLogout = async () => {
    if (!authClient) return;
    try {
      await authClient.logout();
      setIdentity(null);
      setPrincipal(null);
      setBackendActor(null);
      setBalance(null);
      setTransactions([]);
      setError('');
    } catch (error) {
      console.error("Logout failed:", error);
      setError(`Logout failed: ${error.message}`);
    }
  };
 
  const handleDeposit = async () => {
    if (!identity || !depositAmount) {
      setError("Please enter an amount to deposit.");
      return;
    }
    const amount = BigInt(depositAmount);
    if (amount <= 10000n) {
      setError("Deposit amount must be greater than the transaction fee (10,000 e8s).");
      return;
    }
    setIsLoading(true);
    setError('');
    
    try {
      const ledgerAgent = await createLocalAgent(identity);
      const ledgerActor = Actor.createActor(icrcIdlFactory, { 
        agent: ledgerAgent, 
        canisterId: localLedgerCanisterId 
      });
      
      const transferResult = await ledgerActor.icrc1_transfer({
        to: { owner: Principal.fromText(backendCanisterId), subaccount: [] },
        amount: amount,
        fee: [], 
        memo: [], 
        from_subaccount: [], 
        created_at_time: [],
      });
      
      if ('Err' in transferResult) {
        throw new Error(`Ledger transfer failed: ${JSON.stringify(transferResult.Err, jsonReplacer)}`);
      }
      
      const result = await backendActor.deposit(amount);
      if ('Ok' in result) {
        alert(`Deposit of ${Number(amount) / 1e8} LICP successful!`);
        await fetchAllData(backendActor);
        setDepositAmount('');
      } else { 
        throw new Error(`Backend canister error: ${JSON.stringify(result.Err, jsonReplacer)}`); 
      }
    } catch (error) {
      console.error("Deposit error:", error);
      setError(`Error during deposit: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateEscrow = async (e) => {
    e.preventDefault();
    if (!escrowRecipient || !escrowAmount) {
        setError("Please provide a recipient and an amount.");
        return;
    }
    
    let recipientPrincipal;
    try {
      recipientPrincipal = Principal.fromText(escrowRecipient);
    } catch (error) { 
      setError("Invalid recipient principal format."); 
      return; 
    }
    
    const amount = BigInt(escrowAmount);
    setIsLoading(true);
    setError('');
    
    try {
      const request = { 
        to: recipientPrincipal, 
        amount, 
        description: "Frontend Escrow", 
        transaction_type: { 
          'Escrow': { 
            release_conditions: [], 
            auto_release_after: [] 
          } 
        }, 
        currency: { 'ICP': null }, 
        escrow_agent: [], 
        deadline: [], 
        category: [], 
        tags: [], 
      };
      
      const result = await backendActor.create_transaction(request);
      if ('Ok' in result) {
        await fetchAllData(backendActor);
        alert(`Transaction created with ID: ${result.Ok.id}`);
        setEscrowAmount('');
        setEscrowRecipient('');
      } else { 
        throw new Error(`Canister error: ${JSON.stringify(result.Err, jsonReplacer)}`); 
      }
    } catch (error) { 
      console.error("Create escrow error:", error);
      setError(`Error: ${error.message}`); 
    } finally { 
      setIsLoading(false); 
    }
  };

  const handleTransactionAction = async (txId, action) => {
    if (!backendActor) return;
    setIsLoading(true);
    setError('');
    
    try {
        let result;
        if (action === 'accept') {
            result = await backendActor.accept_escrow_terms(txId);
        } else if (action === 'complete') {
            result = await backendActor.complete_transaction(txId);
        } else if (action === 'submit_work') {
            result = await backendActor.submit_escrow_work(txId);
        } else if (action === 'dispute') {
            const reason = prompt("Enter dispute reason:");
            if (reason) {
                result = await backendActor.raise_dispute(txId, reason);
            } else {
                setIsLoading(false);
                return;
            }
        }
        
        if (result && 'Ok' in result) {
            alert(`Action '${action}' successful for transaction ${txId}!`);
            await fetchAllData(backendActor);
        } else if (result) {
            throw new Error(`Canister error: ${JSON.stringify(result.Err, jsonReplacer)}`);
        }
    } catch (error) {
        console.error("Transaction action error:", error);
        setError(`Error performing action: ${error.message}`);
    } finally {
        setIsLoading(false);
    }
  };

  if (isLoading && !principal) {
    return (
      <main style={{ padding: '20px', textAlign: 'center' }}>
        <h1>Loading...</h1>
        <p>Initializing application...</p>
      </main>
    );
  }

  if (!principal) {
    return (
      <main style={{ padding: '20px', textAlign: 'center' }}>
        <h1>Escrow dApp</h1>
        <p>Login with your <strong>local Internet Identity</strong> to test with local test canisters.</p>
        <button 
          onClick={handleLogin} 
          disabled={!authClient}
          style={{
            padding: '12px 24px',
            fontSize: '16px',
            background: '#007bff',
            color: 'white',
            border: 'none',
            borderRadius: '6px',
            cursor: 'pointer',
            marginTop: '20px'
          }}
        >
          Login with Local Internet Identity
        </button>
        {error && (
          <div style={{ 
            color: 'red', 
            marginTop: '20px', 
            padding: '15px', 
            background: '#ffebee', 
            borderRadius: '5px',
            border: '1px solid #ffcdd2',
            maxWidth: '600px',
            margin: '20px auto'
          }}>
            <strong>Error:</strong> {error}
          </div>
        )}
        <div style={{ marginTop: '30px', fontSize: '0.9em', color: '#666', maxWidth: '600px', margin: '30px auto' }}>
          <p><strong>How this works:</strong></p>
          <ul style={{ textAlign: 'left', paddingLeft: '20px' }}>
            <li>You'll authenticate with a local version of Internet Identity running on your machine.</li>
            <li>You will interact with local test canisters also running on your machine.</li>
            <li>This allows you to test with multiple distinct identities without needing a real network connection.</li>
          </ul>
          <p><strong>Prerequisites:</strong></p>
          <ul style={{ textAlign: 'left', paddingLeft: '20px' }}>
            <li>Local replica must be running (<code>dfx start</code>)</li>
            <li>All canisters (backend, ledger, and internet_identity) must be deployed locally</li>
          </ul>
        </div>
      </main>
    );
  }

  return (
    <main style={{ padding: '20px', fontFamily: 'sans-serif', maxWidth: '900px', margin: 'auto' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid #ccc', paddingBottom: '10px' }}>
        <h1>Escrow dApp</h1>
        <button onClick={handleLogout} style={{ padding: '8px 16px', background: '#dc3545', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
          Logout
        </button>
      </div>
      
      <div style={{ margin: '20px 0', padding: '15px', background: '#e8f5e8', borderRadius: '5px' }}>
        <p><strong>Welcome (Local Identity):</strong></p>
        <p style={{ wordBreak: 'break-all', fontSize: '0.9em', fontFamily: 'monospace', background: 'white', padding: '5px', borderRadius: '3px' }}>
          {principal && principal.toText()}
        </p>
        {balance && (
          <p><strong>Local Test Balance:</strong> {Number(balance.available) / 1e8} LICP (Locked: {Number(balance.locked) / 1e8} LICP)</p>
        )}
      </div>
      
      {error && (
        <div style={{ color: 'red', background: '#ffebee', padding: '15px', borderRadius: '5px', marginBottom: '20px', border: '1px solid #ffcdd2' }}>
          <strong>Error:</strong> {error}
        </div>
      )}
      
      <hr />
      
      <section style={{ marginBottom: '30px' }}>
        <h2>1. Deposit Test Funds (from Local Ledger)</h2>
        <div style={{ display: 'flex', gap: '10px', alignItems: 'center', flexWrap: 'wrap' }}>
          <input 
            name="amount" 
            type="number" 
            placeholder="Amount in e8s (1 LICP = 100,000,000)" 
            value={depositAmount} 
            onChange={(e) => setDepositAmount(e.target.value)} 
            required 
            style={{ 
              padding: '8px', 
              borderRadius: '4px', 
              border: '1px solid #ccc',
              minWidth: '250px'
            }} 
          />
          <button 
            onClick={handleDeposit} 
            disabled={isLoading}
            style={{
              padding: '8px 16px',
              background: isLoading ? '#ccc' : '#28a745',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: isLoading ? 'not-allowed' : 'pointer'
            }}
          >
            {isLoading ? 'Processing...' : 'Deposit Test LICP'}
          </button>
        </div>
        <p style={{ fontSize: '0.8em', color: '#666', marginTop: '5px' }}>
          Note: These are test tokens from your local ledger canister, not real ICP.
        </p>
      </section>
      
      <hr />
      
      <section style={{ marginBottom: '30px' }}>
        <h2>2. Create New Escrow Transaction</h2>
        <form onSubmit={handleCreateEscrow} style={{ display: 'flex', flexDirection: 'column', gap: '15px', maxWidth: '500px' }}>
            <input 
              name="recipient" 
              type="text" 
              placeholder="Recipient Principal ID" 
              value={escrowRecipient} 
              onChange={(e) => setEscrowRecipient(e.target.value)} 
              required 
              style={{ padding: '10px', borderRadius: '4px', border: '1px solid #ccc' }} 
            />
            <input 
              name="amount" 
              type="number" 
              placeholder="Amount in e8s (1 LICP = 100,000,000)" 
              value={escrowAmount} 
              onChange={(e) => setEscrowAmount(e.target.value)} 
              required 
              style={{ padding: '10px', borderRadius: '4px', border: '1px solid #ccc' }} 
            />
            <button 
              type="submit" 
              disabled={isLoading} 
              style={{ 
                padding: '12px', 
                background: isLoading ? '#ccc' : '#007bff', 
                color: 'white', 
                border: 'none', 
                borderRadius: '4px', 
                cursor: isLoading ? 'not-allowed' : 'pointer',
                fontSize: '16px'
              }}
            >
              {isLoading ? 'Creating...' : 'Create Escrow'}
            </button>
        </form>
      </section>
      
      <hr />
      
      <section>
        <h2>3. My Transactions</h2>
        {isLoading && <p>Loading transactions...</p>}
        <div style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
            {!isLoading && transactions.length > 0 ? transactions.map(tx => {
                const statusKey = Object.keys(tx.status)[0];
                const isSender = principal && tx.from.toText() === principal.toText();
                const isRecipient = principal && tx.to.toText() === principal.toText();

                return (
                    <div key={Number(tx.id)} style={{ 
                      border: '1px solid #ddd', 
                      padding: '20px', 
                      borderRadius: '8px', 
                      background: '#f9f9f9',
                      boxShadow: '0 2px 4px rgba(0,0,0,0.1)'
                    }}>
                        <div style={{ display: 'grid', gridTemplateColumns: 'auto 1fr', gap: '10px 20px', marginBottom: '15px' }}>
                          <strong>ID:</strong> <span>{Number(tx.id)}</span>
                          <strong>From:</strong> <span style={{fontSize: '0.8em', wordBreak: 'break-all', fontFamily: 'monospace'}}>{tx.from.toText()}</span>
                          <strong>To:</strong> <span style={{fontSize: '0.8em', wordBreak: 'break-all', fontFamily: 'monospace'}}>{tx.to.toText()}</span>
                          <strong>Amount:</strong> <span>{Number(tx.amount) / 1e8} LICP</span>
                          <strong>Status:</strong> <span style={{ 
                            padding: '2px 8px', 
                            borderRadius: '12px', 
                            background: statusKey === 'Completed' ? '#d4edda' : statusKey === 'Pending' ? '#fff3cd' : '#f8d7da',
                            fontSize: '0.9em'
                          }}>{statusKey}</span>
                          <strong>Description:</strong> <span>{tx.description}</span>
                        </div>
                        
                        <div style={{ display: 'flex', gap: '10px', flexWrap: 'wrap' }}>
                          {isRecipient && statusKey === 'Pending' && (
                              <button 
                                onClick={() => handleTransactionAction(tx.id, 'accept')} 
                                disabled={isLoading} 
                                style={{
                                  padding: '8px 16px',
                                  background: '#28a745',
                                  color: 'white',
                                  border: 'none',
                                  borderRadius: '4px',
                                  cursor: isLoading ? 'not-allowed' : 'pointer'
                                }}
                              >
                                Accept Terms
                              </button>
                          )}
                          {isRecipient && statusKey === 'InEscrow' && (
                              <button 
                                onClick={() => handleTransactionAction(tx.id, 'submit_work')} 
                                disabled={isLoading} 
                                style={{
                                  padding: '8px 16px',
                                  background: '#ffc107',
                                  color: 'black',
                                  border: 'none',
                                  borderRadius: '4px',
                                  cursor: isLoading ? 'not-allowed' : 'pointer'
                                }}
                              >
                                Submit Work
                              </button>
                          )}
                          {isSender && (statusKey === 'InEscrow' || statusKey === 'SubmittedForReview') && (
                               <button 
                                 onClick={() => handleTransactionAction(tx.id, 'complete')} 
                                 disabled={isLoading} 
                                 style={{
                                   padding: '8px 16px',
                                   background: '#17a2b8',
                                   color: 'white',
                                   border: 'none',
                                   borderRadius: '4px',
                                   cursor: isLoading ? 'not-allowed' : 'pointer'
                                 }}
                               >
                                 Release Funds
                               </button>
                          )}
                          {(isSender || isRecipient) && (statusKey === 'InEscrow' || statusKey === 'SubmittedForReview') && (
                               <button 
                                 onClick={() => handleTransactionAction(tx.id, 'dispute')} 
                                 disabled={isLoading} 
                                 style={{
                                   padding: '8px 16px',
                                   background: '#dc3545',
                                   color: 'white',
                                   border: 'none',
                                   borderRadius: '4px',
                                   cursor: isLoading ? 'not-allowed' : 'pointer'
                                 }}
                               >
                                 Raise Dispute
                               </button>
                          )}
                        </div>
                    </div>
                );
            }) : !isLoading && <p>No transactions found.</p>}
        </div>
      </section>
    </main>
  );
}

export default App;