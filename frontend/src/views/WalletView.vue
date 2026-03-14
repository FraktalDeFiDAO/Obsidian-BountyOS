<script setup lang="ts">
import { useWalletStore } from '@/stores/wallet'

const wallet = useWalletStore()
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h2 class="text-2xl font-bold mb-6">Wallet</h2>

    <div v-if="!wallet.connected" class="card text-center py-12">
      <p class="text-slate-400 mb-4">Connect your wallet to track balances across chains</p>
      <button @click="wallet.connect" class="btn btn-primary" :disabled="wallet.loading">
        {{ wallet.loading ? 'Connecting...' : 'Connect Wallet' }}
      </button>
    </div>

    <div v-else>
      <div class="card mb-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-slate-400">Connected</p>
            <p class="font-mono text-lg">{{ wallet.shortAddress }}</p>
            <p class="text-sm text-indigo-400">{{ wallet.chain }}</p>
          </div>
          <div class="text-right">
            <p class="text-sm text-slate-400">Total Balance</p>
            <p class="text-2xl font-bold text-green-400">${{ wallet.totalValue.toLocaleString() }}</p>
          </div>
        </div>
        <button @click="wallet.disconnect" class="btn bg-red-700 mt-4">Disconnect</button>
      </div>

      <h3 class="text-lg font-semibold mb-4">Token Balances</h3>
      <div class="grid md:grid-cols-3 gap-4">
        <div v-for="token in wallet.balances" :key="token.symbol" class="card">
          <div class="flex items-center justify-between">
            <span class="font-bold">{{ token.symbol }}</span>
            <span class="text-green-400">${{ token.value.toLocaleString() }}</span>
          </div>
          <p class="text-slate-400 text-sm">{{ token.balance }}</p>
        </div>
      </div>

      <h3 class="text-lg font-semibold mt-8 mb-4">Supported Chains</h3>
      <div class="flex flex-wrap gap-2">
        <span class="px-3 py-1 bg-slate-700 rounded">Ethereum</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Solana</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Bitcoin</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Litecoin</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Dogecoin</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Polygon</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Arbitrum</span>
        <span class="px-3 py-1 bg-slate-700 rounded">Optimism</span>
      </div>
    </div>
  </div>
</template>
