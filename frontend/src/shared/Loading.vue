<template>
    <div class="loading">
        <div>
            <p>{{ loadingText }}</p>
            <button v-if="showCancel" @click="cancel">Abandonner</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { ereaseCookie } from './me';

const dotCount = ref(3)
const loadingText = computed(() => {
    const count = dotCount.value
    let suffix = '.'.repeat(count)
    suffix += ' '.repeat(3 - count)
    return "chargement " + suffix
})

setInterval(() => {
    dotCount.value = (dotCount.value + 1) % 4
}, 500)

const showCancel = ref(false)
setTimeout(() => showCancel.value = true, 2000)

const cancel = () => {
    ereaseCookie()
    window.location.reload()
}
</script>

<style scoped>
.loading {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    backdrop-filter: brightness(110%) blur(5px);
    display: flex;
    align-items: center;
    justify-content: center;
}

.loading p {
    color: black;
    font-size: larger;
    font-weight: bold;
}

.loading div {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>
