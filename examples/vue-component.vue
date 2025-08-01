<template>
  <div>
    <!-- Before: Unsorted classes in Vue template -->
    <div
      class="
        min-h-screen
        bg-gradient-to-br
        from-blue-400
        to-purple-600
        py-12
        px-4
        sm:px-6
        lg:px-8
      "
    >
      <div
        class="
          max-w-4xl
          mx-auto
          bg-white
          rounded-xl
          shadow-2xl
          overflow-hidden
          backdrop-blur-sm
          bg-opacity-95
        "
      >
        <div class="px-8 py-6 bg-gradient-to-r from-blue-500 to-purple-600">
          <h1
            class="
              text-3xl
              font-bold
              text-white
              text-center
              mb-2
              tracking-tight
            "
          >
            Vue Component Example
          </h1>
          <p class="text-blue-100 text-center text-lg">
            Demonstrating Tailwind class sorting in Vue templates
          </p>
        </div>

        <!-- Stats grid -->
        <div class="p-8">
          <div class="grid gap-6 grid-cols-1 md:grid-cols-3">
            <div
              v-for="stat in stats"
              :key="stat.id"
              class="
                p-6
                bg-gray-50
                rounded-lg
                border
                border-gray-200
                text-center
                hover:shadow-md
                transition-shadow
                duration-200
              "
            >
              <div
                class="
                  w-12
                  h-12
                  mx-auto
                  mb-4
                  bg-blue-100
                  rounded-full
                  flex
                  items-center
                  justify-center
                "
              >
                <span class="text-blue-600 font-bold text-xl">
                  {{ stat.icon }}
                </span>
              </div>
              <h3 class="text-2xl font-bold text-gray-900 mb-2">
                {{ stat.value }}
              </h3>
              <p class="text-gray-600 text-sm">
                {{ stat.label }}
              </p>
            </div>
          </div>
        </div>

        <!-- Interactive form -->
        <div class="px-8 pb-8">
          <form @submit.prevent="handleSubmit" class="space-y-6">
            <div class="grid gap-6 grid-cols-1 md:grid-cols-2">
              <div>
                <label
                  for="firstName"
                  class="block text-sm font-medium text-gray-700 mb-2"
                >
                  First Name
                </label>
                <input
                  id="firstName"
                  v-model="form.firstName"
                  type="text"
                  required
                  class="
                    w-full
                    px-4
                    py-3
                    border
                    border-gray-300
                    rounded-lg
                    shadow-sm
                    focus:outline-none
                    focus:ring-2
                    focus:ring-blue-500
                    focus:border-transparent
                    transition-colors
                    duration-200
                  "
                  placeholder="Enter first name"
                />
              </div>

              <div>
                <label
                  for="lastName"
                  class="block text-sm font-medium text-gray-700 mb-2"
                >
                  Last Name
                </label>
                <input
                  id="lastName"
                  v-model="form.lastName"
                  type="text"
                  required
                  class="
                    w-full
                    px-4
                    py-3
                    border
                    border-gray-300
                    rounded-lg
                    shadow-sm
                    focus:outline-none
                    focus:ring-2
                    focus:ring-blue-500
                    focus:border-transparent
                    transition-colors
                    duration-200
                  "
                  placeholder="Enter last name"
                />
              </div>
            </div>

            <div>
              <label
                for="email"
                class="block text-sm font-medium text-gray-700 mb-2"
              >
                Email Address
              </label>
              <input
                id="email"
                v-model="form.email"
                type="email"
                required
                class="
                  w-full
                  px-4
                  py-3
                  border
                  border-gray-300
                  rounded-lg
                  shadow-sm
                  focus:outline-none
                  focus:ring-2
                  focus:ring-blue-500
                  focus:border-transparent
                  transition-colors
                  duration-200
                "
                placeholder="Enter email address"
              />
            </div>

            <div class="flex gap-4 justify-end">
              <button
                type="button"
                @click="resetForm"
                class="
                  px-6
                  py-3
                  text-gray-700
                  bg-gray-200
                  rounded-lg
                  font-medium
                  hover:bg-gray-300
                  focus:outline-none
                  focus:ring-2
                  focus:ring-gray-500
                  focus:ring-offset-2
                  transition-colors
                  duration-200
                "
              >
                Reset
              </button>

              <button
                type="submit"
                :disabled="!isFormValid"
                class="
                  px-8
                  py-3
                  text-white
                  bg-gradient-to-r
                  from-blue-500
                  to-purple-600
                  rounded-lg
                  font-medium
                  hover:from-blue-600
                  hover:to-purple-700
                  focus:outline-none
                  focus:ring-2
                  focus:ring-blue-500
                  focus:ring-offset-2
                  disabled:opacity-50
                  disabled:cursor-not-allowed
                  transition-all
                  duration-200
                  transform
                  hover:scale-105
                "
              >
                Submit
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'VueComponentExample',
  data() {
    return {
      form: {
        firstName: '',
        lastName: '',
        email: '',
      },
      stats: [
        {
          id: 1,
          icon: '⚡',
          value: '99%',
          label: 'Performance',
        },
        {
          id: 2,
          icon: '🎯',
          value: '100%',
          label: 'Accuracy',
        },
        {
          id: 3,
          icon: '🚀',
          value: '24/7',
          label: 'Uptime',
        },
      ],
    };
  },
  computed: {
    isFormValid() {
      return (
        this.form.firstName.trim() &&
        this.form.lastName.trim() &&
        this.form.email.trim()
      );
    },
  },
  methods: {
    handleSubmit() {
      console.log('Form submitted:', this.form);
      // Handle form submission logic here
    },
    resetForm() {
      this.form = {
        firstName: '',
        lastName: '',
        email: '',
      };
    },
  },
};
</script>

<style scoped>
/* Component-specific styles */
</style>