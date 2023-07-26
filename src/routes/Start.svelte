<script lang="ts">
  import {
    Grid,
    Row,
    Column,
    Search,
  } from "carbon-components-svelte";
  import AccTile from "../lib/AccTile.svelte";

  let value = "";

  let recentAccounts = [
		{ id: 'J---aiyznGQ', name: 'Keyboard Cat' },
		{ id: 'z_AbfPXTKms', name: 'Maru' },
		{ id: 'OUtn3pvWmpg', name: 'Henri The Existential Cat' }
	];

  let foundAccounts = [
		{ id: 'J---aiyznGQ', name: 'Keyboard Cat' },
		{ id: 'z_AbfPXTKms', name: 'Maru' },
		{ id: 'OUtn3pvWmpg', name: 'Henri The Existential Cat' }
  ];
</script>


<Grid>
  <Row>
    <Column md={6}>
      <h1>Recent Accounts</h1>
      <Grid noGutter>
        <Row>
          {#each recentAccounts as acc, i}
            <Column md={2}>
              <AccTile acc={acc}/>
            </Column>
          {/each}
        </Row>
      </Grid>
      <br />
      <h1>Search Accounts</h1>
      <Search bind:value size="lg" />
      <br />
      <Grid noGutter>
        <Row>
          {#each foundAccounts as acc, i}
            <Column md={2}>
              <AccTile acc={acc}/>
            </Column>
          {/each}
        </Row>
      </Grid>
    </Column>
    <Column md={2}>
      <h1>Welcome</h1>
    </Column>
  </Row>
</Grid>
