import React from "react";

function Link(props: any): JSX.Element {  
  return (
    <a href={props.href} className="primary-text hover:primary-text-hover cursor-pointer underline">
      {props.children}
    </a>
  );
}

export default Link; 