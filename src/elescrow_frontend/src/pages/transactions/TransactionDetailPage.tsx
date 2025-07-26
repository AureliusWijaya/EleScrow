import React, { useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { Text, Group, ActionIcon } from "@mantine/core";
import Button from "../../shared/components/Button";
import { notifications } from "@mantine/notifications";

interface TransactionDetail {
  id: string;
  date: string;
  time: string;
  from: {
    username: string;
    principal: string;
  };
  to: {
    username: string;
    principal: string;
  };
  description: string;
  amount: number;
  currency: string;
  disputer?: {
    username: string;
    principal: string;
  };
  reason?: string;
}

const mockTransactionDetail: TransactionDetail = {
  id: "3de85225-e6e6-4f63-99a2-aa28d57deb89",
  date: "25 June 2025",
  time: "09.00 GMT +8",
  from: {
    username: "Username",
    principal: "Lbcfr7sAHDC9CgdZo3HTMTkV8MN4ZnX71"
  },
  to: {
    username: "Hsername", 
    principal: "Hlcfr7sDgeC9CgdZo3HTMTkV8MN4ZnX71"
  },
  description: "This is the transaction description, sometimes it explains the reasoning and notes to the receiver",
  amount: 20.00,
  currency: "USDT",
  disputer: {
    username: "Username",
    principal: "Lbcfr7sAHDC9CgdZo3HTMTkV8MN4ZnX71"
  },
  reason: "This is the reason given for the dispute"
};

function TransactionDetailPage(): JSX.Element {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = useState(false);

  const handleApprove = async () => {
    setIsLoading(true);
    try {
      await new Promise(resolve => setTimeout(resolve, 1000));
      notifications.show({
        title: "Transaction Approved",
        message: `Transaction ${id} has been approved successfully`,
        color: "green",
        icon: "✅",
      });
      navigate("/transactions");
    } catch (error) {
      notifications.show({
        title: "Approval Failed",
        message: "Something went wrong. Please try again.",
        color: "red",
        icon: "❌",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleReject = async () => {
    setIsLoading(true);
    try {
      await new Promise(resolve => setTimeout(resolve, 1000));
      notifications.show({
        title: "Transaction Rejected",
        message: `Transaction ${id} has been rejected`,
        color: "red",
        icon: "❌",
      });
      navigate("/transactions");
    } catch (error) {
      notifications.show({
        title: "Rejection Failed",
        message: "Something went wrong. Please try again.",
        color: "red",
        icon: "❌",
      });
    } finally {
      setIsLoading(false);
    }
  };

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text);
    notifications.show({
      title: "Copied!",
      message: `${label} copied to clipboard`,
      color: "blue",
      icon: "📋",
    });
  };

  return (
    <div className="w-full min-h-screen bg-primary p-6">
      <div className="max-w-4xl mx-auto">
        <div className="flex items-center justify-between mb-8">
          <div className="flex items-center gap-4">
            <ActionIcon
              variant="subtle"
              color="gray"
              size="lg"
              onClick={() => navigate("/transactions")}
              className="hover:bg-gray-800/50"
            >
              <i className="bi bi-arrow-left text-xl"></i>
            </ActionIcon>
          </div>
        </div>

        <div className="bg-gray-800/30 rounded-lg p-8">
          <div className="space-y-6">
            <div>
               <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                 Transaction ID
               </Text>
               <Text className="text-primary-text font-mono text-base">
                 {mockTransactionDetail.id}
               </Text>
             </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                             <div>
                 <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                   From
                 </Text>
                 <div className="flex items-center gap-2">
                   <Text className="text-primary-text text-base">
                     {mockTransactionDetail.from.username} ({mockTransactionDetail.from.principal})
                   </Text>
                   <ActionIcon
                     variant="subtle"
                     color="blue"
                     size="sm"
                     onClick={() => copyToClipboard(mockTransactionDetail.from.principal, "From address")}
                   >
                     <i className="bi bi-clipboard"></i>
                   </ActionIcon>
                 </div>
               </div>

               <div>
                 <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                   To
                 </Text>
                 <div className="flex items-center gap-2">
                   <Text className="text-primary-text text-base">
                     {mockTransactionDetail.to.username} ({mockTransactionDetail.to.principal})
                   </Text>
                   <ActionIcon
                     variant="subtle"
                     color="blue"
                     size="sm"
                     onClick={() => copyToClipboard(mockTransactionDetail.to.principal, "To address")}
                   >
                     <i className="bi bi-clipboard"></i>
                   </ActionIcon>
                 </div>
               </div>
            </div>

                         <div>
               <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                 Description
               </Text>
               <Text className="text-primary-text text-base">
                 {mockTransactionDetail.description}
               </Text>
             </div>

             <div>
               <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                 Amount
               </Text>
               <Text className="text-incoming-text text-lg font-semibold">
                 {mockTransactionDetail.currency} +{mockTransactionDetail.amount.toFixed(2)}
               </Text>
             </div>

            {mockTransactionDetail.disputer && (
              <>
                                 <div>
                   <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                     Disputer
                   </Text>
                   <div className="flex items-center gap-2">
                     <Text className="text-primary-text text-base">
                       {mockTransactionDetail.disputer.username} ({mockTransactionDetail.disputer.principal})
                     </Text>
                     <ActionIcon
                       variant="subtle"
                       color="blue"
                       size="sm"
                       onClick={() => copyToClipboard(mockTransactionDetail.disputer!.principal, "Disputer address")}
                     >
                       <i className="bi bi-clipboard"></i>
                     </ActionIcon>
                   </div>
                 </div>

                 <div>
                   <Text className="text-primary-text font-bold text-xl border-b border-gray-600 pb-2 mb-3">
                     Reason Given
                   </Text>
                   <Text className="text-primary-text text-base">
                     {mockTransactionDetail.reason}
                   </Text>
                 </div>
              </>
            )}
          </div>

          <div className="flex gap-4 mt-8 pt-6 border-t border-gray-600">
            <Button
              click={handleApprove}
              className={`!px-12 !py-3 !text-lg !font-semibold ${isLoading ? 'opacity-50 cursor-not-allowed' : ''}`}
            >
              {isLoading ? (
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  Approving...
                </div>
              ) : (
                'Approve'
              )}
            </Button>
            <Button
              variant="outlined"
              click={handleReject}
              className={`!px-12 !py-3 !text-lg !font-semibold ${isLoading ? 'opacity-50 cursor-not-allowed' : ''}`}
            >
              {isLoading ? (
                <div className="flex items-center gap-2">
                  <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  Rejecting...
                </div>
              ) : (
                'Reject'
              )}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default TransactionDetailPage;