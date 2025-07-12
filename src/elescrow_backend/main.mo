actor {
  public type Portfolio = {
    totalValue: Float;
    change24h: Float;
  };

  public query func greet(name : Text) : async Text {
    return "Hello, " # name # "!";
  };

  public query func getPortfolio() : async Portfolio {
    return {
      totalValue = 10523.45;
      change24h = 1.5;
    };
  };
}