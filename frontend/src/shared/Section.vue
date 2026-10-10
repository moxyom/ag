<template>
    <component :is="is" class="section">
        <div class="page">
            <slot></slot>
        </div>
    </component>
</template>

<script lang="ts" setup>
defineProps<{ is: string }>()
</script>

<style>
.section {
    background: var(--color-primary-transparent-2);
    padding: 8px 0;
}

.section:not(:last-child) {
    margin-bottom: 32px;
}

.section .page {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
</style>
