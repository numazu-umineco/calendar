<template>
  <v-app>
    <v-app-bar color="blue">
      <v-container>
        <v-row>
          <v-col>
            <v-app-bar-title class="font-weight-bold">Umineco Calendar System</v-app-bar-title>
          </v-col>
        </v-row>
      </v-container>
    </v-app-bar>

    <v-container class="content">
      <v-row>
        <v-col>
          <router-view />
        </v-col>
      </v-row>
    </v-container>
  </v-app>
</template>

<style scoped>
.content {
  margin-top: 64px;
}
</style>
