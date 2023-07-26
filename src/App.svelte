<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import UiShell from './lib/UIShell.svelte';
  import Start from './routes/Start.svelte';
  import Card from "./routes/Card.svelte";
  export let url = "/";
</script>

<Router {url}>
  <UiShell>
    <Route path="/card" component={Card} />
    <Route path="/"><Start /></Route>
  </UiShell>
</Router>
