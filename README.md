# `EleScrow`

EleScrow is a decentralized escrow platform designed to ensure safe and trustless transactions between two parties, especially in freelance or gig-based environments. In many existing platforms, there's a common risk: once the service provider delivers the work, the client may disappear or refuse to pay. We aim to eliminate this risk by automating the escrow process on-chain.

Using the Internet Computer, we’ve built a smart contract that securely holds the buyer's funds and only releases them when both parties agree the job is complete. If disputes arise, additional logic (or optional arbitration modules) can resolve them. This protects both the buyer and seller, making the platform safe, transparent, and reliable.

We’re also integrating features like messaging to support smoother collaboration throughout the transaction lifecycle.

# How to Start EleScrow

Do `npm install` on the root folder to install dependencies, then `dfx start --clean --background` and `dfx deploy` on the root folder to start the backend of EleScrow. To start the frontend, navigate to elescrow_frontend folder and do `npm run dev`.
