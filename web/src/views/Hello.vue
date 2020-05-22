<template>
    <v-app id="inspire">
          <v-content>
              
              <v-card>

                  <v-row justify="space-between">
        <v-col cols="auto">
          <v-img
              src="https://secure.gravatar.com/avatar/1dc4720d84eca6d5ea0efefce4fdda5f.jpg?s=80&r=g&d=mm"
              class="mt-10 ml-10"
              width="100"
            ></v-img>
        </v-col>

        <v-col
        >
                  
            <v-list-item three-line>
      <v-list-item-content class="ma-6">
        <div class="overline mb-4">Welcome</div>
        <v-list-item-title class="headline mb-1">Layer5</v-list-item-title>
        <v-list-item-subtitle class=" mb-2">The Service Mesh Company</v-list-item-subtitle>
        <v-list-item-content>Lorem ipsum dolor sit amet, consectetur adipiscing elit</v-list-item-content>

      </v-list-item-content>
    </v-list-item>
        </v-col>
                  </v-row>
          </v-card>


<v-card
    class="mx-auto mt-10"
    max-width="60%"
  >
  <v-simple-table class="pa-10">
    <template v-slot:default>
      <thead>
        <tr>
          <th class="text-left">Image Name</th>
          <th class="text-left"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in images" :key="item">
          <td>{{ item }}</td>
          <td>
              <v-btn icon @click="pullImage">
                <v-icon>mdi-download</v-icon>
              </v-btn>
          </td>
        </tr>
      </tbody>
    </template>
  </v-simple-table>
  </v-card>
  </v-content>
  <v-dialog
      v-model="dialog"
      width="500"
    >

      <v-card>
        <v-card-title
          class="headline grey lighten-2"
          primary-title
        >
          {{dialogTitle}}
        </v-card-title>

        <v-card-text>
          {{dialogBody}}
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="primary"
            text
            @click="upgrade"
          >
            Upgrade
          </v-btn>
          <v-btn
            color="primary"
            text
            @click="dialog = false"
          >
            Cancel
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    </v-app>
</template>

<script>
import axios from 'axios';

export default {
    created() {
        console.log('Component has been created!');
        
    },
    data: () => ({
        message: "",
        images: [
            "layer5 / meshery",
            "layer5 / meshery-nsm",
            "layer5 / meshery-consul",
            "layer5 / meshery-octarine",
            "layer5 / meshery-istio",
            "layer5 / meshery-linkerd",
            "layer5 / meshery-cloud",
            "layer5 / meshery-cpx",
            "layer5 / meshery-maesh",
            "layer5 / meshery-kuma",
            "layer5 / meshery-app-mesh",
            "layer5 / meshery-nsx-sm",
            ],
        dialog: false,
        dialogTitle: "",
        dialogBody: "",
    }),
  methods: {
    pullImage: function () {
        console.log("Done!")
        let token = localStorage.getItem("token")
        if (token == null) {
            this.message = "Unauthorized"
            return
        }
        let userObj = JSON.parse(atob(token))
        axios.get(this.$BASE_URL + "/hello", {
            headers: {
                Authorization: token
            }
        }).then((res) => {
            this.message = `Hello ${userObj.username} (${userObj.plan}) !` 
            
            this.dialogTitle = `Hello ${userObj.username}!`
            this.dialogBody = `The image has been successfully pulled! you are subsribed to ${userObj.plan} plan, please click on the upgrade button to upgrade.`
            this.dialog = true
            
            console.log(res)
        }).catch((res) => {
            console.log(res)
            if (res.response.status == 429) {
                this.dialogTitle = `Sorry ${userObj.username}!`
                this.dialogBody = `The image pull failed as you have exceeded your limit! you are subsribed to ${userObj.plan} plan, please click on the upgrade button to upgrade.`
                this.dialog = true
                // this.message = "Limit Exceeded"
            } else {
                this.dialogTitle = `Please Login / Signup!`
                this.dialogBody = `You are unauthorized to pull images, please login to continue.`
                this.dialog = true
            }
        })
    },
    upgrade: function () {
        this.dialog = false
        this.$router.push({ name: 'Upgrade'})
    }
  }
}
</script>