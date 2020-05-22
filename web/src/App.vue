<template>
  <v-app id="inspire">
    <v-app-bar
      app
      color="indigo"
      dark
    >

      <v-toolbar-title><router-link :to="{name: 'Hello'}">
      <v-btn text>
        Home
      </v-btn>
        </router-link>
      </v-toolbar-title>
      
      <v-spacer></v-spacer>
      <router-link :to="{name: 'Signup'}">
      <v-btn text>
        Sign Up
      </v-btn>
        </router-link>
      <router-link :to="{name: 'Auth'}">
      <v-btn text>
        Login
      </v-btn>
      </router-link>
      <v-btn text @click="logout">
        Logout
      </v-btn>
    </v-app-bar>
    <v-content>
    <router-view/>
    </v-content>
  </v-app>
</template>

<script>
  export default {
    props: {
      source: String,
    },
    data: () => ({
    }),
    methods: {
      logout: function () {
        localStorage.removeItem("token")
        location.reload()
      }
    }
  }
</script>