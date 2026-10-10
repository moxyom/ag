<template>
    <div class="toggle" @click="joinable = !joinable">
        <p>
            Inscriptions 
            <span v-if="joinable" class="open">ouvertes</span>
            <span v-else class="close">fermées</span>
        </p>
    </div>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue"

const emits = defineEmits<{
    (event: "on-change", open: boolean): void
}>()

const props = defineProps<{ joinable: boolean }>()
const joinable = ref(props.joinable)
watch(props, () => joinable.value = props.joinable)

watch(joinable, (newValue) => emits("on-change", newValue))
</script>


<style scoped>
.toggle {
    padding: 4px 16px;
    background: var(--color-primary-transparent);
    border-radius: 2px;
    cursor: pointer;
}

.toggle p {
    margin: -4px;
    padding: -4px;
}

.open {
    color: rgb(8, 129, 8);
}

.close {
    color: rgb(207, 13, 13);
}
</style>
