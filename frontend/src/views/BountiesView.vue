<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Bounty {
  id: string
  title: string
  platform: string
  status: string
  description: string
  url: string
  bounty_type: string
  reward_min?: number
  reward_max?: number
  reward_currency?: string
  skills: string[]
  tags: string[]
}

const bounties = ref<Bounty[]>([])
const loading = ref(true)
const error = ref('')
const platform = ref('')
const status = ref('')

const platforms = ['github', 'gitcoin', 'hackerone', 'bugcrowd', 'laborx', 'dework']
const statuses = ['active', 'closed', 'expired', 'draft', 'paused']

const fetchBounties = async () => {
  loading.value = true
  error.value = ''
  
  try {
    const response = await fetch('/api/bounties')
    if (!response.ok) throw new Error('Failed to fetch')
    const data = await response.json()
    bounties.value = data.bounties || []
  } catch {
    // Demo data when API unavailable
    bounties.value = [
      { id: '1', title: 'Fix security vulnerability in auth', platform: 'github', status: 'active', description: 'Critical security issue', url: '#', bounty_type: 'bugbounty', skills: ['rust'], tags: ['security'] },
      { id: '2', title: 'Add dark mode support', platform: 'github', status: 'active', description: 'Implement dark mode', url: '#', bounty_type: 'task', skills: ['vue'], tags: ['ui'] },
      { id: '3', title: 'DeFi Grant Program', platform: 'gitcoin', status: 'active', description: 'Grant for DeFi', url: '#', bounty_type: 'grant', skills: ['solidity'], tags: ['defi'] },
    ]
  } finally {
    loading.value = false
  }
}

const formatReward = (min?: number, max?: number) => {
  if (!min && !max) return 'Unspecified'
  const c = 'USD'
  if (min && max && min !== max) return `$${min}-${max} ${c}`
  if (min) return `$${min}+ ${c}`
  if (max) return `Up to $${max} ${c}`
  return 'Unspecified'
}

const getPlatformColor = (p: string) => {
  const colors: Record<string, string> = { github: 'bg-gray-700', gitcoin: 'bg-green-700', hackerone: 'bg-red-700', bugcrowd: 'bg-orange-700', laborx: 'bg-blue-700', dework: 'bg-purple-700' }
  return colors[p] || 'bg-gray-700'
}

onMounted(fetchBounties)
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <div class="flex flex-wrap justify-between gap-4 mb-6">
      <h2 class="text-2xl font-bold">Bounties</h2>
      <div class="flex gap-3">
        <select v-model="platform" @change="fetchBounties" class="input w-auto">
          <option value="">All Platforms</option>
          <option v-for="p in platforms" :key="p" :value="p">{{ p }}</option>
        </select>
        <select v-model="status" @change="fetchBounties" class="input w-auto">
          <option value="">All Status</option>
          <option v-for="s in statuses" :key="s" :value="s">{{ s }}</option>
        </select>
        <button @click="fetchBounties" class="btn bg-slate-700">Refresh</button>
      </div>
    </div>

    <div v-if="loading" class="text-center py-12">
      <div class="animate-spin w-8 h-8 border-4 border-indigo-500 border-t-transparent rounded-full mx-auto"></div>
    </div>

    <div v-else-if="bounties.length === 0" class="text-center py-12">
      <p class="text-slate-400">No bounties found</p>
    </div>

    <div v-else class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
      <a v-for="bounty in bounties" :key="bounty.id" :href="bounty.url" target="_blank" 
         class="card hover:border-indigo-500 border border-transparent">
        <div class="flex justify-between mb-2">
          <span :class="['px-2 py-1 rounded text-xs', getPlatformColor(bounty.platform)]">{{ bounty.platform }}</span>
          <span class="px-2 py-1 rounded text-xs bg-green-900 text-green-300">{{ bounty.status }}</span>
        </div>
        <h3 class="font-semibold mb-2">{{ bounty.title }}</h3>
        <p class="text-sm text-slate-400 mb-2">{{ bounty.description }}</p>
        <div class="flex justify-between text-sm">
          <span class="text-indigo-400">{{ formatReward(bounty.reward_min, bounty.reward_max) }}</span>
        </div>
      </a>
    </div>
  </div>
</template>
