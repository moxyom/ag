```vue
<template>
    <div class="wrapper">
        <div class="room-id-input">
            <template v-for="(_, index) in values" :key="index">
                <input :ref="(el) => setInputRef(el, index)" :value="values[index]" type="text" inputmode="numeric"
                pattern="[0-9]*" maxlength="1" autocomplete="one-time-code" :aria-label="`Chiffre ${index + 1}`"
                @input="onInput($event, index)" @keydown="onKeydown($event, index)" @paste="onPaste($event, index)" />
                
                <span v-if="index === 2" class="separator">-</span>
            </template>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const values = ref<string[]>(Array(6).fill(''))
const inputs = ref<HTMLInputElement[]>([])

const emit = defineEmits<{
    (event: "complete", code: number): void
}>()

function setInputRef(el: unknown, index: number) {
    if (el instanceof HTMLInputElement) {
        inputs.value[index] = el
    }
}

function onInput(event: Event, index: number) {
    const input = event.target as HTMLInputElement

    // On conserve uniquement le premier chiffre valide
    const digit = input.value.replace(/\D/g, '').slice(0, 1)

    values.value[index] = digit
    input.value = digit

    if (digit && index < 5) {
        inputs.value[index + 1]?.focus()
    }

    if (values.value.every((value) => value !== '')) {
        emit('complete', Number(values.value.join('')))
        values.value = Array(6).fill('')
        inputs.value[0].focus()
    }
}

function onKeydown(event: KeyboardEvent, index: number) {
    if (
        event.key === 'Backspace' &&
        !values.value[index] &&
        index > 0
    ) {
        inputs.value[index - 1]?.focus()
        return
    }

    if (event.key === 'ArrowLeft' && index > 0) {
        event.preventDefault()
        inputs.value[index - 1]?.focus()
    }

    if (event.key === 'ArrowRight' && index < 5) {
        event.preventDefault()
        inputs.value[index + 1]?.focus()
    }
}

function onPaste(event: ClipboardEvent, index: number) {
    event.preventDefault()

    const pasted = event.clipboardData
        ?.getData('text')
        .replace(/\D/g, '')

    if (!pasted) return

    const digits = pasted.slice(0, 6 - index).split('')

    digits.forEach((digit, offset) => {
        values.value[index + offset] = digit
    })

    const nextIndex = Math.min(index + digits.length, 5)
    inputs.value[nextIndex]?.focus()

    if (values.value.every((value) => value !== '')) {
        emit('complete', Number(values.value.join('')))
    }
}
</script>

<style scoped>
.room-id-input {
    display: flex;
    gap: 8px;
    align-items: center;
    justify-content: baseline;
}

.room-id-input span {
    color: var(--color-primary);
    font-size: 40px;
}

.room-id-input input {
    width: 16px;
    font-size: 30px;
    text-align: center;
    color: black;
    padding: 8px 16px;
}
</style>
