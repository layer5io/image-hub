<template>
    <v-content>
<v-card
    class="mx-auto"
    max-width="80%"
    flat
  >
    <v-container fluid>
      <v-row dense>
        <v-col
          v-for="card in cards"
          :key="card.title"
          cols="4"
        >
          <v-card class="fill-height">
            <v-list-item three-line>
      <v-list-item-content class="ma-6">
        <div class="overline mb-4">{{card.price}}</div>
        <v-list-item-title class="headline mb-1">{{card.plan}}</v-list-item-title>
        <v-list-item-subtitle class=" mb-2">{{card.rpm}}</v-list-item-subtitle>
        <v-list-item-content>{{card.content}}</v-list-item-content>

      </v-list-item-content>
    </v-list-item>
    <v-card-actions>
      <v-btn text @click="upgrade(card.plan)">upgrade</v-btn>
    </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </v-card>
    </v-content>
</template>

<script>
import axios from 'axios';
  export default {
    data: () => ({
      cards: [
        { price: 'FREE', plan: 'Personal', rpm: '10 pulls / min', content: 'The basics of Docker for every developer, including unlimited public repos and one private repo.'},
        { price: '$10/month', plan: 'Team', rpm: '100 pulls / min', content: 'Pulls and tools for a development team popular projects, including unlimited public repos and ten private repos.'},
        { price: '$100/month', plan: 'Enterprise', rpm: 'Unlimited', content: 'Unlimited pulls and advanced tools for all your teams and their popular projects, including unlimited public repos and a thousand private repos.'},
      ],
    }),
    methods: {
    upgrade: function (upgradePlan) {
      let token = localStorage.getItem("token")
      let userObj = JSON.parse(atob(token))
      axios.post(this.$BASE_URL + "/upgrade",{
        username: userObj.username,
        plan: upgradePlan
      }).then((res) => {
        console.log(res)
        localStorage.setItem("token", res.data["token"])
        this.$router.push({ name: 'Hello'})
      }).catch((res) => {
        console.log(res)
      })
    }
  }
  }
</script>