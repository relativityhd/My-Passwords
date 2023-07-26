<script>
  import {
    Header,
    SkipToContent,
    Content
  } from "carbon-components-svelte";

  let isSideNavOpen = false;
</script>

<Header company="IBM" platformName="Carbon Svelte" bind:isSideNavOpen>
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
</Header>

<Content>
  <slot />
</Content>