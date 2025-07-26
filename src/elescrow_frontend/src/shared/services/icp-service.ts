import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";

// Import the actual backend interface from the generated declarations
import { idlFactory, _SERVICE } from '../../../';

interface CreateTransactionRequest {
  to: Principal;
  transaction_type: { DirectPayment: null } | { Escrow: { release_conditions: string[], auto_release_after?: bigint } };
  amount: bigint;
  currency: { ICP: null } | { USDT: null } | { Custom: { symbol: string, decimals: number } };
  description: string;
  escrow_agent?: Principal;
  deadline?: bigint;
  category?: { Business: null } | { Personal: null } | { Investment: null } | { Salary: null } | { Freelance: null } | { Refund: null } | { Other: { name: string } };
  tags: string[];
}

interface Result<T> {
  Ok?: T;
  Err?: any;
}

class ICPService {
  private agent: HttpAgent | null = null;
  private actor: _SERVICE | null = null;
  private authClient: AuthClient | null = null;
  private canisterId: string = "";

  async initialize() {
    try {
      this.authClient = await AuthClient.create();
      const identity = this.authClient.getIdentity();
      
      this.agent = new HttpAgent({
        identity,
        host: process.env.DFX_NETWORK === "ic" ? "https://ic0.app" : "http://localhost:4943"
      });

      if (process.env.DFX_NETWORK !== "ic") {
        await this.agent.fetchRootKey();
      }

      // Get canister ID from environment or use default
      this.canisterId = process.env.CANISTER_ID_ELESCROW_BACKEND || "uxrrr-q7777-77774-qaaaq-cai";

      // Create real actor using the generated declarations
      this.actor = Actor.createActor<_SERVICE>(idlFactory, {
        agent: this.agent,
        canisterId: this.canisterId,
      });

      console.log("Using real backend actor with canister ID:", this.canisterId);
      return true;
    } catch (error) {
      console.error("Failed to initialize ICP service:", error);
      return false;
    }
  }

  async login() {
    if (!this.authClient) {
      throw new Error("Auth client not initialized");
    }

    return new Promise<Principal>((resolve, reject) => {
      this.authClient!.login({
        identityProvider: process.env.DFX_NETWORK === "ic" 
          ? "https://identity.ic0.app/#authorize"
          : "http://localhost:4943?canisterId=rdmx6-jaaaa-aaaaa-aaadq-cai",
        onSuccess: () => {
          const principal = this.authClient!.getIdentity().getPrincipal();
          resolve(principal);
        },
        onError: (error) => {
          reject(error);
        }
      });
    });
  }

  async logout() {
    if (!this.authClient) {
      throw new Error("Auth client not initialized");
    }
    
    try {
      await this.authClient.logout();
      this.agent = null;
      this.actor = null;
      this.authClient = null;
      return true;
    } catch (error) {
      console.error("Logout failed:", error);
      throw error;
    }
  }

  getActor() {
    if (!this.actor) {
      throw new Error("Actor not initialized. Call initialize() first.");
    }
    return this.actor;
  }

  getPrincipal(): Principal | null {
    if (!this.authClient) return null;
    return this.authClient.getIdentity().getPrincipal();
  }

  isAuthenticated(): boolean {
    return this.authClient?.isAuthenticated() || false;
  }

  async createTransaction(request: CreateTransactionRequest): Promise<Result<any>> {
    try {
      const actor = this.getActor();
      const result = await actor.create_transaction(request);
      return { Ok: result };
    } catch (error: any) {
      console.error("Create transaction error:", error);
      return { Err: error.message || "Failed to create transaction" };
    }
  }

  async getBalance(): Promise<Result<any>> {
    try {
      const actor = this.getActor();
      const result = await actor.get_balance();
      return { Ok: result };
    } catch (error: any) {
      console.error("Get balance error:", error);
      return { Err: error.message || "Failed to get balance" };
    }
  }

  async getUserByUsername(username: string): Promise<Result<any>> {
    try {
      const actor = this.getActor();
      const result = await actor.search_users({
        verification_level: [],
        query: [username],
        created_after: [],
        is_active: [],
        created_before: [],
      }, {
        offset: 0n,
        limit: 10n,
      });
      
      if (result.Ok && result.Ok.length > 0) {
        return { Ok: result.Ok[0] };
      } else {
        return { Err: "User not found" };
      }
    } catch (error: any) {
      console.error("Get user by username error:", error);
      return { Err: error.message || "Failed to find user" };
    }
  }

  async getMyTransactions(): Promise<Result<any[]>> {
    try {
      const actor = this.getActor();
      const result = await actor.get_my_transactions([], {
        offset: 0n,
        limit: 50n,
      });
      return { Ok: result.Ok || [] };
    } catch (error: any) {
      console.error("Get transactions error:", error);
      return { Err: error.message || "Failed to get transactions" };
    }
  }
}

export const icpService = new ICPService();