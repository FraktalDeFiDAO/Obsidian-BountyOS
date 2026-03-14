<script setup lang="ts">
import { ref } from 'vue'

interface Bounty {
  id: string
  title: string
  platform: string
  status: string
  reward: string
}

const bounties = ref<Bounty[]>([])
const loading = ref(false)

const platforms = ['github', 'gitcoin', 'hackerone', 'bugcrowd', 'laborx', 'dework']
const selectedPlatform = ref<string>('')
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-2xl font-bold">Bounties</h2>
      <div class="flex gap-4">
        <select v-model="selectedPlatform" class="input w-auto">
          <option value="">All Platforms</option>
          <option v-for="p in platforms" :key="p" :value="p">{{ p }}</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="text-center py-12">
      <p class="text-slate-400">Loading bounties...</p>
    </div>

    <div v-else-if="bounties.length === 0" class="text-center py-12">
      <p class="text-slate-400 mb-4">No bounties found</p>
      <p class="text-sm text-slate-500">Run the CLI to scan for bounties: obsidian-bounty-finder scan --all</p>
    </div>

    <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div v-for="bounty in bounties" :key="bounty.id" class="card hover:border-indigo-500 border border-transparent">
        <h3 class="font-semibold mb-2">{{ bounty.title }}</h3>
        <div class="flex items-center justify-between text-sm text-slate-400">
          <span class="capitalize">{{ bounty.platform }}</span>
          <span class="px-2 py-1 rounded text-xs bg-green-900 text-green-300">{{ bounty.status }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
