<script>
  import {
    Header,
    SkipToContent,
    Content
  } from "carbon-components-svelte";

</script>

<Header company="My" platformName="Passwords" href="/">
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
</Header>

<Content>
  <slot />
</Content>