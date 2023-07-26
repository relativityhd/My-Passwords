<script lang="ts">
  import {
    ClickableTile
  } from "carbon-components-svelte";
  import { Link } from "svelte-routing";
  import type { Account } from "../types/account"

  export let acc: Account;
</script>

<Link to="/card">
<ClickableTile>{acc.name}</ClickableTile>
</Link>
