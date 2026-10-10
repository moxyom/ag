<template>
    <div class="notice" v-if="show">
        <CloseIcon @click="isClosed = true" />
        <p>{{ message }}</p>
    </div>
</template>

<script lang="ts" setup>
import { ref, watch, computed } from 'vue'
import CloseIcon from './CloseIcon.vue'

const props = defineProps<{ message: string }>()

const isClosed = ref(false)
watch(props, () => isClosed.value = false)

const show = computed(() => props.message.length > 0 && !isClosed.value)
</script>

<style scoped>
.notice {
    position: fixed;
    bottom: 16px;
    left: 50%;
    transform: translateX(-50%);
    padding: 8px 24px 8px 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    background: var(--color-primary-transparent);
    border-radius: 8px;
}

.notice svg {
    cursor: pointer;
}
</style>
