<template>
    <v-content>
      <v-container
        class="fill-height"
        fluid
      >
        <v-row
          align="center"
          justify="center"
        >
          <v-col
            cols="12"
            sm="8"
            md="4"
          >
            <v-card class="elevation-12">
              <v-toolbar
                color="#3C494F"
                dark
                flat
              >
                <v-toolbar-title>Login</v-toolbar-title>
                <v-spacer></v-spacer>
                <!-- <v-tooltip bottom>
                  <template v-slot:activator="{ on }">
                    <v-btn
                      :href="source"
                      icon
                      large
                      target="_blank"
                      v-on="on"
                    >
                      <v-icon>mdi-code-tags</v-icon>
                    </v-btn>
                  </template>
                  <span>Source</span>
                </v-tooltip> -->
              </v-toolbar>
              <v-card-text>
                <v-form>
                  <v-text-field
                    label="username"
                    name="username"
                    v-model="username"
                    type="text"
                  ></v-text-field>

                  <v-text-field
                    id="password"
                    label="Password"
                    name="password"
                    v-model="password"
                    type="password"
                  ></v-text-field>
                </v-form>
              </v-card-text>
              <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="#00D3A9" @click="login">Login</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-content>
</template>

<script>
// import HelloWorld from './components/HelloWorld';
import axios from 'axios';

export default {
  name: 'App',
  created() {
    console.log('Component has been created!');
    },
  data: () => ({
    username: "",
    password: "",
  }),
  methods: {
    login: function () {
      axios.post(this.$BASE_URL + "/auth",{
        username: this.username,
        password: this.password
      }).then((res) => {
        console.log(res)
        localStorage.setItem("token", res.data["token"])
        this.$router.push({ name: 'Hello'})
      }).catch((res) => {
        console.log(res)
      })
    }
  }
};
</script>
