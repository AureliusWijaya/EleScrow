import React from "react";
import { Slide, ToastContainer } from "react-toastify";

function CustomToastContainer(): JSX.Element {
    return (
        <ToastContainer
            position="top-right"
            autoClose={1000}
            hideProgressBar={false}
            newestOnTop
            closeOnClick
            rtl={false}
            pauseOnFocusLoss
            draggable={false}
            pauseOnHover={false}
            theme="dark"
            transition={Slide}
        ></ToastContainer>
    );
}

export default CustomToastContainer;
