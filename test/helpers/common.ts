import { ApiPromise } from "@polkadot/api";
import { u32 } from "@polkadot/types";
import Bottleneck from "bottleneck";
import { DevModeContext, importJsonConfig, MoonwallContext } from "@moonwall/cli";
import { ethers, Signer } from "ethers";
import fetch from "node-fetch";
export function rateLimiter() {
  const settings =
    process.env.SKIP_RATE_LIMITER === "true" ? {} : { maxConcurrent: 10, minTime: 150 };
  return new Bottleneck(settings);
}

export async function checkTimeSliceForUpgrades(
  api: ApiPromise,
  blockNumbers: number[],
  currentVersion: u32
) {
  const apiAt = await api.at(await api.rpc.chain.getBlockHash(blockNumbers[0]));
  const onChainRt = (await apiAt.query.system.lastRuntimeUpgrade()).unwrap().specVersion;
  return { result: !onChainRt.eq(currentVersion), specVersion: onChainRt };
}

// Sort dict by key
export function sortObjectByKeys(o) {
  return Object.keys(o)
    .sort()
    .reduce((r, k) => ((r[k] = o[k]), r), {});
}

interface JsonRpcResponse {
  result?: any;
  error?: {
    code: number;
    message: string;
  };
}

export async function customDevRpcRequest(method: string, params: any[]) {
  const globalConfig = await importJsonConfig();
  const env = globalConfig.environments.find(({ name }) => name == process.env.MOON_TEST_ENV)!;
  const endpoint = env.connections
    ? env.connections[0].endpoints[0].replace("ws://", "http://")
    : `http://127.0.0.1:${10000 + Number(process.env.VITEST_POOL_ID || 1) * 100}`;
  const data = {
    jsonrpc: "2.0",
    id: 1,
    method,
    params,
  };

  const response = await fetch(endpoint, {
    method: "POST",
    body: JSON.stringify(data),
    headers: { "Content-Type": "application/json" },
  });

  const responseData: JsonRpcResponse = await response.json();

  if (responseData.error) {
    throw new Error(responseData.error.message);
  }

  return responseData.result;
}

export async function getMappingInfo(
  context: DevModeContext,
  authorId: string
): Promise<{ account: string; deposit: BigInt }> {
  const mapping = await context
    .polkadotJs({ type: "moon" })
    .query.authorMapping.mappingWithDeposit(authorId);
  if (mapping.isSome) {
    return {
      account: mapping.unwrap().account.toString(),
      deposit: mapping.unwrap().deposit.toBigInt(),
    };
  }
  return null;
}
