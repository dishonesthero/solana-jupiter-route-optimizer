import { describe, it } from "node:test";
import assert from "node:assert";
import { RouteEvaluator, SolanaJupiterRouteClient, SwapHop } from "../src/index";

describe("Solana Jupiter Route Optimizer Test Suite", () => {
  const evaluator = new RouteEvaluator();

  it("should select the route with highest net output after fee deduction", () => {
    const route1: SwapHop[] = [
      { dexName: "Raydium", inputMint: "SOL", outputMint: "USDC", expectedOut: 10000n, feeBps: 25 },
    ];
    const route2: SwapHop[] = [
      { dexName: "Whirlpool", inputMint: "SOL", outputMint: "USDC", expectedOut: 10200n, feeBps: 30 },
    ];

    const best = evaluator.findBestRoute([route1, route2]);
    assert.strictEqual(best[0].dexName, "Whirlpool");
  });

  it("should enforce minimum required output under slippage constraint", () => {
    const client = new SolanaJupiterRouteClient();
    assert.strictEqual(client.verifyMinimumOutput(10000n, 9950n, 100), true); // 100 bps = 1%
    assert.strictEqual(client.verifyMinimumOutput(10000n, 9850n, 100), false);
  });
});
