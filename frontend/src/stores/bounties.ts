import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface Bounty {
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

export const useBountyStore = defineStore('bounties', () => {
  const bounties = ref<Bounty[]>([])
  const loading = ref(false)
  const error = ref('')
  const selectedPlatform = ref('')
  const selectedStatus = ref('')

  const filteredBounties = computed(() => {
    return bounties.value.filter(b => {
      const platformMatch = !selectedPlatform.value || b.platform === selectedPlatform.value
      const statusMatch = !selectedStatus.value || b.status === selectedStatus.value
      return platformMatch && statusMatch
    })
  })

  const stats = computed(() => ({
    total: bounties.value.length,
    active: bounties.value.filter(b => b.status === 'active').length,
    platforms: [...new Set(bounties.value.map(b => b.platform))],
  }))

  async function fetchBounties() {
    loading.value = true
    error.value = ''
    try {
      const res = await fetch('/api/bounties')
      const data = await res.json()
      bounties.value = data.bounties || []
    } catch (e: any) {
      error.value = e.message
      // Demo data
      bounties.value = [
        { id: '1', title: 'Security Fix', platform: 'github', status: 'active', description: 'Critical bug', url: '#', bounty_type: 'bugbounty', skills: ['rust'], tags: [] },
      ]
    } finally {
      loading.value = false
    }
  }

  function setPlatform(platform: string) {
    selectedPlatform.value = platform
  }

  function setStatus(status: string) {
    selectedStatus.value = status
  }

  return { bounties, loading, error, selectedPlatform, selectedStatus, filteredBounties, stats, fetchBounties, setPlatform, setStatus }
})
