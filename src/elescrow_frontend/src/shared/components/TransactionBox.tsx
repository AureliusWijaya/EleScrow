import React from "react";

function TransactionBox(props: any): JSX.Element {
  const getBackground = () => {
    if (props.type === "outgoing") {
      return "bg-outgoing";
    }
    return "bg-incoming";
  };

  const getAmountColor = () => {
    if (props.type === "outgoing") {
      return "text-outgoing-text";
    }
    return "text-incoming-text";
  };

  const formatAmount = () => {
    const sign = props.type === "outgoing" ? "-" : "+";
    return `${sign}${props.amount}`;
  };

  return (
    <div className={`rounded-lg p-4 w-full primary-text ${getBackground()}`}>
      <div className="flex items-center gap-2 mb-2">
        <i className="bi bi-calendar text-sm"></i>
        <span className="text-sm">{props.date}</span>
      </div>
      
      <div className="mb-3">
        <h3 className="font-semibold text-base">{props.username}</h3>
        <p className="text-sm opacity-80">{props.description}</p>
      </div>

      <div className="mb-2">
        <span className="text-sm capitalize">{props.type}</span>
      </div>

      <div className="flex items-center justify-between">
        <span className="font-bold text-lg">{props.currency}</span>
        <span className={`font-bold text-lg ${getAmountColor()}`}>
          {formatAmount()}
        </span>
      </div>
    </div>
  );
}

export default TransactionBox; 