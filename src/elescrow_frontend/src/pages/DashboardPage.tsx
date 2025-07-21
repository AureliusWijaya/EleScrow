import React from "react";
import TransactionBox from "../shared/components/TransactionBox";

function DashboardPage(): JSX.Element {
  const recentTransactions = [
    {
      date: "13 July 2025",
      username: "Username",
      description: "Transaction Description",
      type: "incoming",
      currency: "USDT",
      amount: "30.00"
    },
    {
      date: "13 July 2025",
      username: "Username",
      description: "Transaction Description",
      type: "incoming",
      currency: "USDT",
      amount: "30.00"
    },
    {
      date: "13 July 2025", 
      username: "Username",
      description: "Transaction Description",
      type: "incoming",
      currency: "USDT",
      amount: "30.00"
    }
  ];

  const activeTransactions = [
    {
      date: "13 July 2025",
      username: "Username", 
      description: "Transaction Description",
      type: "incoming",
      currency: "USDT",
      amount: "30.00"
    },
    {
      date: "13 July 2025",
      username: "Username",
      description: "Transaction Description", 
      type: "incoming",
      currency: "USDT",
      amount: "30.00"
    },
    {
      date: "13 July 2025",
      username: "Username",
      description: "Transaction Description",
      type: "outgoing", 
      currency: "USDT",
      amount: "30.00"
    },
    {
      date: "13 July 2025",
      username: "Username",
      description: "Transaction Description",
      type: "incoming",
      currency: "USDT", 
      amount: "30.00"
    }
  ];

  return (
    <div className="min-h-screen bg-primary p-6">
      <div className="max-w-6xl mx-auto">
        <div className="mb-8">
          <div className="flex-column items-center justify-between mb-4">
            <h2 className="text-2xl text-primary-text">Recent Transactions</h2>
            <span className="text-primary-text-hover text-sm cursor-pointer hover:text-secondary-hover">→ Details</span>
          </div>
          
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-4 gap-4">
            {recentTransactions.map((transaction, index) => (
              <TransactionBox
                key={index}
                date={transaction.date}
                username={transaction.username}
                description={transaction.description}
                type={transaction.type}
                currency={transaction.currency}
                amount={transaction.amount}
              />
            ))}
          </div>
        </div>

        <div>
          <div className="flex-column items-center justify-between mb-4">
            <h2 className="text-2xl text-primary-text">Active Transactions</h2>
            <span className="text-primary-text-hover text-sm cursor-pointer hover:text-secondary-hover">→ Details</span>
          </div>
          
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-4 gap-4">
            {activeTransactions.map((transaction, index) => (
              <TransactionBox
                key={index}
                date={transaction.date}
                username={transaction.username}
                description={transaction.description}
                type={transaction.type}
                currency={transaction.currency}
                amount={transaction.amount}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

export default DashboardPage; 