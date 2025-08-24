import React, { useState } from "react";
import Button from "../../shared/components/Button";
import Input from "../../shared/components/input/Input";
import Select from "../../shared/components/Select";
import Textarea from "../../shared/components/Textarea";
import { notifications } from "@mantine/notifications";
import Form from "../../shared/components/form/Form";
import SubmitButton from "../../shared/components/SubmitButton";
import { FormValidators } from "../../shared/utils/form/form-validators.c";

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
                message:
                    "Please fill in all fields before creating the transaction",
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

            await new Promise((resolve) => setTimeout(resolve, 1000));

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

                    <Form
                        submit={isLoading ? undefined : handleCreateTransaction}
                    >
                        <div className="flex flex-col gap-6">
                            <div className="relative">
                                <Input
                                    fcId="username"
                                    label="Search for User"
                                    type="text"
                                    placeholder="Username"
                                    validators={[FormValidators.required]}
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
                                    fcId="currency"
                                    label="Currency"
                                    className="min-w-[120px] cursor-pointer"
                                    validators={[FormValidators.required]}
                                >
                                    <option value="USDT" className="bg-primary">
                                        USDT
                                    </option>
                                    <option value="BTC" className="bg-primary">
                                        BTC
                                    </option>
                                    <option value="ETH" className="bg-primary">
                                        ETH
                                    </option>
                                    <option value="ICP" className="bg-primary">
                                        ICP
                                    </option>
                                </Select>

                                <Input
                                    fcId="amount"
                                    label="Amount"
                                    type="number"
                                    placeholder="Amount"
                                    className="flex-1"
                                    validators={[
                                        FormValidators.required,
                                        FormValidators.min(1),
                                    ]}
                                />
                            </div>

                            <Textarea
                                fcId="description"
                                label="Description"
                                placeholder="Description"
                                rows={6}
                            />
                        </div>

                        <SubmitButton
                            className={`w-full !py-4 !text-base !font-semibold ${
                                isLoading ? "opacity-50 cursor-not-allowed" : ""
                            }`}
                        >
                            {isLoading ? (
                                <div className="flex items-center justify-center gap-2">
                                    <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                                    Creating...
                                </div>
                            ) : (
                                "Create"
                            )}
                        </SubmitButton>
                    </Form>
                </div>
            </div>
        </div>
    );
}

export default CreateTransactionPage;
