#!/usr/bin/env ts-node
import "@composable/types/augment-api";
import "@composable/types/augment-types";
import { getNewConnection } from "@composable/utils";
import { verifyCrowdloanData } from "@composable/crowdloan_data_verifier/handler";


const CONTRIBUTOR_LIST_URL =
  "https://raw.githubusercontent.com/ComposableFi/composable/main/frontend/apps/picasso/defi/polkadot/constants/pica-rewards-contributions.json";

const main = async () => {
  console.log("Crowdloan Pallet Verifier");

  console.log("Connecting...");
  // Establish connection to the node.
  const endpoint = process.env.ENDPOINT ?? "ws://127.0.0.1:9988";
  const { newClient } = await getNewConnection(endpoint);

  const contributorsUrl = process.env.CONTRIBUTOR_LIST_URL ?? CONTRIBUTOR_LIST_URL;

  // Here the actual magic happens
  // @ts-ignore
  await verifyCrowdloanData(newClient, contributorsUrl);

  // Disconnecting from the node.
  console.debug("disconnecting...");
  await newClient.disconnect();
};

main()
  .then(() => {
    console.log("Crowdloan data verification finished!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
