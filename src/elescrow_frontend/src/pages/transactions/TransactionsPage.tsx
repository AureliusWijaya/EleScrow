import React, { useState, useMemo } from "react";
import { Table, Tabs, TextInput, ActionIcon, Badge, Group, Text, Modal } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import Button from "../../shared/components/Button";
import { notifications } from "@mantine/notifications";

interface Transaction {
  id: string;
  date: string;
  user: string;
  amount: number;
  currency: string;
  status: string;
  type: 'incoming' | 'outgoing';
}

const mockTransactions: Transaction[] = [
  {
    id: "3de85225-e6e6-4f63-99a2-aa28d57deb89",
    date: "12 July 2025",
    user: "Username",
    amount: 20.00,
    currency: "USDT",
    status: "Waiting for Both Approval",
    type: "incoming"
  },
  {
    id: "5ac50f9d-18bb-47df-8595-30f398cf41c0",
    date: "11 July 2025", 
    user: "Username",
    amount: 20.00,
    currency: "USDT",
    status: "Rejected by Other User",
    type: "incoming"
  },
  {
    id: "d8ae8635-43b9-4b8c-9ad6-0495c255a3c6",
    date: "11 July 2025",
    user: "Username", 
    amount: 20.00,
    currency: "USDT",
    status: "Waiting for Other Approval",
    type: "outgoing"
  },
  {
    id: "d8ae8635-43b9-4b8c-9ad6-0495c255a3c7",
    date: "11 July 2025",
    user: "Username",
    amount: 20.00, 
    currency: "USDT",
    status: "Waiting for Your Approval",
    type: "outgoing"
  }
];

function TransactionsPage(): JSX.Element {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<string>("incoming");
  const [searchQuery, setSearchQuery] = useState("");
  const [sortBy, setSortBy] = useState<"date" | "amount">("date");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("desc");
  const [modalOpened, setModalOpened] = useState(false);
  const [selectedAction, setSelectedAction] = useState<string>("");
  const [selectedTransactionId, setSelectedTransactionId] = useState<string>("");

  const getStatusColor = (status: string) => {
    switch (status) {
      case "Waiting for Both Approval":
        return "yellow";
      case "Rejected by Other User": 
        return "red";
      case "Waiting for Other Approval":
        return "orange";
      case "Waiting for Your Approval":
        return "blue";
      default:
        return "gray";
    }
  };

  const handleAction = (action: string, transactionId: string) => {
    if (action === "view") {
      navigate(`/transaction/${transactionId}`);
    } else if (action === "approve" || action === "reject") {
      setSelectedAction(action);
      setSelectedTransactionId(transactionId);
      setModalOpened(true);
    } else {
      notifications.show({
        title: `${action} Action`,
        message: `${action} performed on transaction ${transactionId}`,
        color: action === "protect" ? "orange" : "blue",
      });
    }
  };

  const handleConfirmAction = () => {
    notifications.show({
      title: `Transaction ${selectedAction === "approve" ? "Approved" : "Rejected"}`,
      message: `Transaction ${selectedTransactionId} has been ${selectedAction === "approve" ? "approved" : "rejected"}`,
      color: selectedAction === "approve" ? "green" : "red",
      icon: selectedAction === "approve" ? "✅" : "❌",
    });
    setModalOpened(false);
    setSelectedAction("");
    setSelectedTransactionId("");
  };

  const handleCancelAction = () => {
    setModalOpened(false);
    setSelectedAction("");
    setSelectedTransactionId("");
  };

  const filteredTransactions = useMemo(() => {
    return mockTransactions
      .filter(transaction => transaction.type === activeTab)
      .filter(transaction => 
        transaction.id.toLowerCase().includes(searchQuery.toLowerCase()) ||
        transaction.user.toLowerCase().includes(searchQuery.toLowerCase()) ||
        transaction.status.toLowerCase().includes(searchQuery.toLowerCase())
      )
      .sort((a, b) => {
        if (sortBy === "date") {
          const comparison = new Date(a.date).getTime() - new Date(b.date).getTime();
          return sortOrder === "asc" ? comparison : -comparison;
        } else {
          const comparison = a.amount - b.amount;
          return sortOrder === "asc" ? comparison : -comparison;
        }
      });
  }, [activeTab, searchQuery, sortBy, sortOrder]);

  const incomingCount = mockTransactions.filter(t => t.type === "incoming").length;
  const outgoingCount = mockTransactions.filter(t => t.type === "outgoing").length;

  const rows = filteredTransactions.map((transaction) => (
    <Table.Tr key={transaction.id} className="hover:bg-gray-800/50">
      <Table.Td className="text-secondary-text text-sm font-mono">
        {transaction.id}
      </Table.Td>
      <Table.Td className="text-primary-text">
        {transaction.date}
      </Table.Td>
      <Table.Td className="text-primary-text">
        {transaction.user}
      </Table.Td>
      <Table.Td className="text-primary-text">
        <span className={transaction.type === "incoming" ? "text-incoming-text" : "text-outgoing-text"}>
          {transaction.type === "incoming" ? "+" : "-"}{transaction.currency} {transaction.amount.toFixed(2)}
        </span>
      </Table.Td>
      <Table.Td>
        <Badge 
          color={getStatusColor(transaction.status)}
          variant="light"
          size="sm"
        >
          {transaction.status}
        </Badge>
      </Table.Td>
      <Table.Td>
        <Group gap="xs">
          <ActionIcon
            variant="subtle"
            color="blue"
            size="sm"
            onClick={() => handleAction("view", transaction.id)}
          >
            <i className="bi bi-eye"></i>
          </ActionIcon>
          <ActionIcon
            variant="subtle"
            color="green"
            size="sm"
            onClick={() => handleAction("approve", transaction.id)}
          >
            <i className="bi bi-check"></i>
          </ActionIcon>
          <ActionIcon
            variant="subtle"
            color="red"
            size="sm"
            onClick={() => handleAction("reject", transaction.id)}
          >
            <i className="bi bi-x"></i>
          </ActionIcon>
          <ActionIcon
            variant="subtle"
            color="orange"
            size="sm"
            onClick={() => handleAction("protect", transaction.id)}
          >
            <i className="bi bi-shield"></i>
          </ActionIcon>
        </Group>
      </Table.Td>
    </Table.Tr>
  ));

  return (
    <div className="w-full min-h-screen bg-primary p-6">
      <div className="max-w-7xl mx-auto">
        <div className="flex items-center justify-between mb-6">
          <Text size="xl" className="text-primary-text font-semibold">
            Transactions ({mockTransactions.length})
          </Text>
          <Button click={() => navigate("/transaction/create")}>
            Create a Transaction
          </Button>
        </div>

        <div className="bg-gray-800/30 rounded-lg p-4">
          <div className="flex items-center justify-between mb-4">
            <Tabs 
              value={activeTab} 
              onChange={(value) => setActiveTab(value || "incoming")}
              classNames={{
                tab: "text-primary-text hover:text-secondary data-[active=true]:text-secondary",
                tabLabel: "text-sm font-medium"
              }}
            >
              <Tabs.List className="bg-transparent border-b border-gray-600">
                <Tabs.Tab value="incoming">
                  Incoming ({incomingCount})
                </Tabs.Tab>
                <Tabs.Tab value="outgoing">
                  Outgoing ({outgoingCount})
                </Tabs.Tab>
              </Tabs.List>
            </Tabs>

            <TextInput
              placeholder="Search for Transaction"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.currentTarget.value)}
              rightSection={<i className="bi bi-search text-secondary-text"></i>}
              className="w-80"
              classNames={{
                input: "bg-transparent border-secondary-text text-primary-text placeholder-secondary-text"
              }}
            />
          </div>

          <Table 
            striped={false} 
            highlightOnHover={false}
            className="w-full"
            classNames={{
              th: "text-primary-text border-b border-gray-600 py-3",
              td: "border-b border-gray-700/50 py-3"
            }}
          >
            <Table.Thead>
              <Table.Tr>
                <Table.Th className="text-primary-text">Transaction ID</Table.Th>
                <Table.Th 
                  className="text-primary-text cursor-pointer hover:text-secondary"
                  onClick={() => {
                    if (sortBy === "date") {
                      setSortOrder(sortOrder === "asc" ? "desc" : "asc");
                    } else {
                      setSortBy("date");
                      setSortOrder("desc");
                    }
                  }}
                >
                  <Group gap="xs">
                    Transaction Date
                    <i className={`bi bi-arrow-${sortBy === "date" && sortOrder === "desc" ? "down" : "up"}`}></i>
                  </Group>
                </Table.Th>
                <Table.Th className="text-primary-text">User</Table.Th>
                <Table.Th className="text-primary-text">Amount</Table.Th>
                <Table.Th className="text-primary-text">Status</Table.Th>
                <Table.Th className="text-primary-text">Actions</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody className="bg-slate-700">{rows}</Table.Tbody>
          </Table>

                     {filteredTransactions.length === 0 && (
             <div className="text-center py-8">
               <Text className="text-secondary-text">
                 No transactions found
               </Text>
             </div>
           )}
         </div>

         <Modal
           opened={modalOpened}
           onClose={handleCancelAction}
           title=""
           centered
           size="md"
           overlayProps={{
             backgroundOpacity: 0.55,
             blur: 3,
           }}

         >
           <div className="text-center p-6">
             <Text size="xl" className="text-primary-text mb-6 font-semibold">
               <span className="text-secondary capitalize">{selectedAction}</span> the transaction
             </Text>
             <Text className="text-primary-text text-lg mb-8 font-mono">
               {selectedTransactionId}?
             </Text>
             
             <div className="flex gap-4 justify-center">
               <Button
                 click={handleConfirmAction}
                 className="!px-12 !py-3 !text-lg !font-semibold"
               >
                 Yes
               </Button>
               <Button
                 variant="outlined"
                 click={handleCancelAction}
                 className="!px-12 !py-3 !text-lg !font-semibold"
               >
                 No
               </Button>
             </div>
           </div>
         </Modal>
       </div>
     </div>
   );
 }

export default TransactionsPage; 