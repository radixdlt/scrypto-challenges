<template>
  <v-container>
    <v-card variant="outlined">
      <v-card-title> Import account from PBE plugin</v-card-title>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="importAccountFromPlugin()"
        >
          Import Account
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress" variant="outlined">
      <v-card-title>
        (Optional) Create new Lendi Platform component
      </v-card-title>
      <v-card-text>
        This will create a new Lendi platform component where you will be
        granted an admin badge
      </v-card-text>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            createNewLendiPlatformComponent(accountAddress, packageAddress)
          "
        >
          Create new component
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress && adminBadgeAddress" variant="outlined">
      <v-card-title> Add an new asset to Lendi Platform</v-card-title>
      <v-card-item>
        <p>For reference, the XRD Resource Address is: {{ xrdAddress }}</p>
        <v-text-field
          v-model="newResourceAddress"
          label="Enter Resource Address to Add"
        >
        </v-text-field>
        <v-text-field
          v-model="loanToValueRatio"
          label="Enter Loan to Value Ratio"
        >
        </v-text-field>
      </v-card-item>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            addNewAsset(
              lendingComponentAddress,
              accountAddress,
              adminBadgeAddress,
              xrdAddress,
              loanToValueRatio
            )
          "
        >
          Add new asset
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress" variant="outlined">
      <v-card-title> Create Lendi Account</v-card-title>
      <v-card-text>
        WARNING: Creating a new account will remove any previous accounts from
        your web browser!
      </v-card-text>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="createLendiAccount(lendingComponentAddress, accountAddress)"
        >
          Create Account
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress && badgeAddress" variant="outlined">
      <v-card-text>
        <p>For reference, the XRD Resource Address is: {{ xrdAddress }}</p>
      </v-card-text>
    </v-card>
    <v-card v-if="accountAddress && badgeAddress" variant="outlined">
      <v-card-title>Check Asset Balance</v-card-title>
      <v-card-text>
        <v-text-field
          v-model="checkBalancesResourceAddress"
          label="Enter Resource Address to Check Balances For"
        >
        </v-text-field>
        <p v-if="depositBalance">Deposit Balance: {{ depositBalance }}</p>
        <p v-if="borrowBalance">Borrow Balance: {{ borrowBalance }}</p>
      </v-card-text>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            checkBalances(
              lendingComponentAddress,
              accountAddress,
              badgeAddress,
              checkBalancesResourceAddress
            )
          "
        >
          Check Balances
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress && badgeAddress" variant="outlined">
      <v-card-title> Deposit Assets</v-card-title>
      <v-card-item>
        <v-text-field
          v-model="depositResourceAddress"
          label="Enter Resource Address to Deposit"
        ></v-text-field>
        <v-text-field v-model="depositAmount" label="Enter Amount to Deposit">
        </v-text-field>
      </v-card-item>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            depositAsset(
              lendingComponentAddress,
              accountAddress,
              badgeAddress,
              depositResourceAddress,
              depositAmount
            )
          "
        >
          Deposit
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress && badgeAddress" variant="outlined">
      <v-card-title>Withdraw Assets</v-card-title>
      <v-card-item>
        <v-text-field
          v-model="withdrawResourceAddress"
          label="Enter Resource Address to Withdraw"
        ></v-text-field>
        <v-text-field v-model="withdrawAmount" label="Enter Amount to Withdraw">
        </v-text-field>
      </v-card-item>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            withdrawAsset(
              lendingComponentAddress,
              accountAddress,
              badgeAddress,
              withdrawResourceAddress,
              withdrawAmount
            )
          "
        >
          Withdraw
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress && badgeAddress" variant="outlined">
      <v-card-title> Borrow Assets</v-card-title>
      <v-card-item>
        <v-text-field
          v-model="borrowResourceAddress"
          label="Enter Resource Address to Borrow"
        ></v-text-field>
        <v-text-field v-model="borrowAmount" label="Enter Amount to Borrow">
        </v-text-field>
      </v-card-item>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            borrowAsset(
              lendingComponentAddress,
              accountAddress,
              badgeAddress,
              borrowResourceAddress,
              borrowAmount
            )
          "
        >
          Borrow
        </v-btn>
      </v-card-actions>
    </v-card>
    <v-card v-if="accountAddress && badgeAddress" variant="outlined">
      <v-card-title> Repay Assets</v-card-title>
      <v-card-item>
        <v-text-field
          v-model="repayResourceAddress"
          label="Enter Resource Address to Repay"
        ></v-text-field>
        <v-text-field v-model="repayAmount" label="Enter Amount to Repay">
        </v-text-field>
      </v-card-item>
      <v-card-actions>
        <v-btn
          color="primary"
          variant="outlined"
          @click="
            repayAsset(
              lendingComponentAddress,
              accountAddress,
              badgeAddress,
              repayResourceAddress,
              repayAmount
            )
          "
        >
          Repay
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import { ManifestBuilder } from "pte-sdk";
import { getAccountAddress, signTransaction } from "pte-browser-extension-sdk";

interface Data {
  xrdAddress: string;
  packageAddress: string;
  lendingComponentAddress: string;
  accountAddress?: string;
  badgeAddress?: string;
  adminBadgeAddress?: string;
  newResourceAddress?: string;
  loanToValueRatio: number;
  checkBalancesResourceAddress?: string;
  depositBalance?: string;
  borrowBalance?: string;
  depositResourceAddress?: string;
  depositAmount: number;
  withdrawResourceAddress?: string;
  withdrawAmount: number;
  borrowResourceAddress?: string;
  borrowAmount: number;
  repayResourceAddress?: string;
  repayAmount: number;
}

export default defineComponent({
  name: "HomeView",
  data(): Data {
    return {
      xrdAddress: "030000000000000000000000000000000000000000000000000004",
      packageAddress: "01007e454127802c5a6180f55a69083ed13f12625feca4a1efbce2",
      lendingComponentAddress:
        "026b8eac48b4209b01a58d03cdc0864d4c2fe23070ca512bf31c55",
      accountAddress: undefined,
      badgeAddress: undefined,
      adminBadgeAddress: undefined,
      newResourceAddress: undefined,
      loanToValueRatio: 0.0,
      checkBalancesResourceAddress: undefined,
      depositBalance: undefined,
      borrowBalance: undefined,
      depositResourceAddress: undefined,
      depositAmount: 0.0,
      withdrawResourceAddress: undefined,
      withdrawAmount: 0.0,
      borrowResourceAddress: undefined,
      borrowAmount: 0.0,
      repayResourceAddress: undefined,
      repayAmount: 0.0,
    };
  },
  methods: {
    async importAccountFromPlugin() {
      this.accountAddress = await getAccountAddress();
      this.badgeAddress = undefined;
      this.adminBadgeAddress = undefined;
    },
    async createNewLendiPlatformComponent(
      accountAddress: string,
      packageAddress: string
    ) {
      const manifest = new ManifestBuilder()
        .callFunction(
          packageAddress,
          "LendingPlatform",
          "instantiate_lending_platform",
          []
        )
        .callMethodWithAllResources(accountAddress, "deposit_batch")
        .build()
        .toString();

      const receipt = await signTransaction(manifest);

      if (receipt.newComponents && receipt.newComponents.length > 0) {
        this.lendingComponentAddress = receipt.newComponents[0];
      }

      if (receipt.newResources && receipt.newResources.length > 0) {
        this.adminBadgeAddress = receipt.newResources[0];
      }

      console.log(receipt);
    },
    async addNewAsset(
      lendingComponentAddress: string,
      accountAddress: string,
      adminBadgeAddress: string,
      assetAddress: string,
      loanToValueRatio: number
    ) {
      const proofName = "admin_badge_proof";

      const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, adminBadgeAddress)
        .createProofFromAuthZone(adminBadgeAddress, proofName)
        .callMethod(lendingComponentAddress, "new_asset", [
          `ResourceAddress("${assetAddress}")`,
          `Decimal("${loanToValueRatio}")`,
        ])
        .build()
        .toString();

      const receipt = await signTransaction(manifest);
      console.log(receipt);
    },
    async createLendiAccount(
      lendingComponentAddress: string,
      accountAddress: string
    ) {
      const manifest = new ManifestBuilder()
        .callMethod(lendingComponentAddress, "new_user", [])
        .callMethodWithAllResources(accountAddress, "deposit_batch")
        .build()
        .toString();

      const receipt = await signTransaction(manifest);
      console.log(receipt);
      const resources = receipt.newResources;
      this.badgeAddress = resources.pop();
    },
    async checkBalances(
      lendingComponentAddress: string,
      accountAddress: string,
      badgeAddress: string,
      assetAddress: string
    ) {
      const proofName1 = "user_badge_proof_1";
      const proofName2 = "user_badge_proof_2";

      const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, badgeAddress)
        .createProofFromAuthZone(badgeAddress, proofName1)
        .createProofFromAuthZone(badgeAddress, proofName2)
        .callMethod(lendingComponentAddress, "get_resource_deposit_balance", [
          `ResourceAddress("${assetAddress}")`,
          `Proof("${proofName2}")`,
        ])
        .callMethod(lendingComponentAddress, "get_resource_borrow_balance", [
          `ResourceAddress("${assetAddress}")`,
          `Proof("${proofName1}")`,
        ])
        .build()
        .toString();

      const receipt = await signTransaction(manifest);

      const outputs = receipt.outputs;
      if (outputs && outputs.length == 5) {
        console.log(outputs[3], outputs[4]);

        const depositBalanceStringRaw = outputs[3];
        const borrowBalanceStringRaw = outputs[4];

        const valueRegex = /\\"(.*?)\\/gm;

        const depositBalanceMatch = depositBalanceStringRaw.match(valueRegex);
        let depositBalance =
          depositBalanceMatch && depositBalanceMatch.length > 0
            ? depositBalanceMatch[0]
            : "";
        depositBalance = depositBalance.replace(/\\/g, "");
        depositBalance = depositBalance.replace('"', "");
        console.log("depositBalance", depositBalance);
        this.depositBalance = depositBalance; // TODO make this a number
        console.log("depositBalance", depositBalance);

        const borrowBalanceMatch = borrowBalanceStringRaw.match(valueRegex);
        let borrowBalance =
          borrowBalanceMatch && borrowBalanceMatch.length > 0
            ? borrowBalanceMatch[0]
            : "";
        borrowBalance = borrowBalance.replace(/\\/g, "");
        borrowBalance = borrowBalance.replace('"', "");
        console.log("borrowBalance", borrowBalance);
        this.borrowBalance = borrowBalance; // TODO make this a number
        console.log("borrowBalance", borrowBalance);
      }
      console.log(receipt);
    },
    async depositAsset(
      lendingComponentAddress: string,
      accountAddress: string,
      badgeAddress: string,
      assetAddress: string,
      amount: number
    ) {
      const proofName = "user_badge_proof";
      const assetsBucketName = "assets_bucket";

      const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, badgeAddress)
        .createProofFromAuthZone(badgeAddress, proofName)
        .withdrawFromAccountByAmount(accountAddress, amount, assetAddress)
        .takeFromWorktopByAmount(amount, assetAddress, assetsBucketName)
        .callMethod(lendingComponentAddress, "deposit_asset", [
          `Bucket("${assetsBucketName}")`,
          `Proof("${proofName}")`,
        ])
        .callMethodWithAllResources(accountAddress, "deposit_batch")
        .build()
        .toString();

      const receipt = await signTransaction(manifest);
      console.log(receipt);
    },
    async withdrawAsset(
      lendingComponentAddress: string,
      accountAddress: string,
      badgeAddress: string,
      assetAddress: string,
      amount: number
    ) {
      const proofName = "user_badge_proof";

      const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, badgeAddress)
        .createProofFromAuthZone(badgeAddress, proofName)
        .callMethod(lendingComponentAddress, "withdraw_asset", [
          `ResourceAddress("${assetAddress}")`,
          `Decimal("${amount}")`,
          `Proof("${proofName}")`,
        ])
        .callMethodWithAllResources(accountAddress, "deposit_batch")
        .build()
        .toString();

      const receipt = await signTransaction(manifest);
      console.log(receipt);
    },
    async borrowAsset(
      lendingComponentAddress: string,
      accountAddress: string,
      badgeAddress: string,
      assetAddress: string,
      amount: number
    ) {
      const proofName = "asdf";

      const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, badgeAddress)
        .createProofFromAuthZone(badgeAddress, proofName)
        .callMethod(lendingComponentAddress, "borrow_asset", [
          `ResourceAddress("${assetAddress}")`,
          `Decimal("${amount}")`,
          `Proof("${proofName}")`,
        ])
        .callMethodWithAllResources(accountAddress, "deposit_batch")
        .build()
        .toString();

      const receipt = await signTransaction(manifest);
      console.log(receipt);
    },
    async repayAsset(
      lendingComponentAddress: string,
      accountAddress: string,
      badgeAddress: string,
      assetAddress: string,
      amount: number
    ) {
      const proofName = "user_badge_proof";
      const assetsBucketName = "assets_bucket";

      const manifest = new ManifestBuilder()
        .createProofFromAccount(accountAddress, badgeAddress)
        .createProofFromAuthZone(badgeAddress, proofName)
        .withdrawFromAccountByAmount(accountAddress, amount, assetAddress)
        .takeFromWorktopByAmount(amount, assetAddress, assetsBucketName)
        .callMethod(lendingComponentAddress, "repay_asset", [
          `Bucket("${assetsBucketName}")`,
          `Proof("${proofName}")`,
        ])
        .callMethodWithAllResources(accountAddress, "deposit_batch")
        .build()
        .toString();

      const receipt = await signTransaction(manifest);
      console.log(receipt);
    },
  },
});
</script>
