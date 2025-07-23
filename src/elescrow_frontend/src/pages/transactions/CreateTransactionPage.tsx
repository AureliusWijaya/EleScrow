import React, { useState } from "react";
import Button from "../../shared/components/Button";
import Input from "../../shared/components/Input";
import Select from "../../shared/components/Select";
import Textarea from "../../shared/components/Textarea";
import { notifications } from "@mantine/notifications";

function CreateTransactionPage(): JSX.Element {
  const [username, setUsername] = useState("");
  const [amount, setAmount] = useState("");
  const [currency, setCurrency] = useState("USDT");
  const [description, setDescription] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSearchUser = () => {
    if (!username.trim()) {
      notifications.show({
        title: "Username Required",
        message: "Please enter a username to search",
        color: "red",
        icon: "❌",
      });
      return;
    }
    
    console.log("Searching for user:", username);
    notifications.show({
      title: "Searching...",
      message: `Looking for user: ${username}`,
      color: "blue",
      icon: "🔍",
    });
  };

  const handleCreateTransaction = async () => {
    if (!username || !amount || !description) {
      notifications.show({
        title: "Missing Information",
        message: "Please fill in all fields before creating the transaction",
        color: "red",
        icon: "⚠️",
      });
      return;
    }

    if (parseFloat(amount) <= 0) {
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
      console.log("Creating transaction:", {
        username,
        amount,
        currency,
        description,
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      notifications.show({
        title: "Transaction Created Successfully! 🎉",
        message: `${amount} ${currency} transaction created for ${username}`,
        color: "green",
        icon: "✅",
        autoClose: 5000,
      });

      setUsername("");
      setAmount("");
      setDescription("");
    } catch (error) {
      notifications.show({
        title: "Transaction Failed",
        message: "Something went wrong. Please try again.",
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
                className="absolute right-3 bottom-3 text-secondary-text hover:text-secondary transition-colors"
              >
                <i className="bi bi-search text-lg"></i>
              </button>
            </div>

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
            className={`w-full !py-4 !text-base !font-semibold ${isLoading ? 'opacity-50 cursor-not-allowed' : ''}`}
            click={isLoading ? undefined : handleCreateTransaction}
          >
            {isLoading ? (
              <div className="flex items-center justify-center gap-2">
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                Creating...
              </div>
            ) : (
              'Create'
            )}
          </Button>
        </div>
      </div>
    </div>
  );
}

export default CreateTransactionPage; 