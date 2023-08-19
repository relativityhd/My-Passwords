<script>
	import { CopyButton } from 'carbon-components-svelte';
	import { Grid, Row, Column } from 'carbon-components-svelte';
	import { Breadcrumb, BreadcrumbItem } from 'carbon-components-svelte';
	import {
		StructuredList,
		StructuredListRow,
		StructuredListCell,
		StructuredListBody
	} from 'carbon-components-svelte';
	import { Tile } from 'carbon-components-svelte';
	import { Tag } from 'carbon-components-svelte';
	import { Button } from 'carbon-components-svelte';
	import { FaceActivated } from 'carbon-icons-svelte';
	import TrashCan from 'carbon-icons-svelte/lib/TrashCan.svelte';
	import { DateTime } from 'luxon';

	export let data;
</script>

<!--
  How Carbon Grid works:
  sm=4
  md=8
  lg=16
  xlg=16
  max=16
-->

<Grid>
	<Row>
		<Column sm={4}>
			<Breadcrumb noTrailingSlash>
				<BreadcrumbItem href="/">Home</BreadcrumbItem>
				<BreadcrumbItem href="/password/{data.id}" isCurrentPage>
					{data.institution_name}:{data.account_name}
				</BreadcrumbItem>
			</Breadcrumb>
		</Column>
	</Row>
	<Row>
		<Column sm={4} md={8} lg={7}>
			<h1 style="display: inline;">{data.password}</h1>
			<CopyButton style="display: inline;" text={data.password} feedback="Copied to clipboard" />
			<br />
			{#if false}
				<Tag type="magenta">Legacy Secure</Tag>
			{:else if false}
				<Tag type="green">Super Secure</Tag>
			{:else}
				<Tag type="teal">Secure</Tag>
			{/if}
		</Column>
		<Column sm={4} md={4} lg={5}>
			<h3>{data.institution_name}</h3>
			<p>Website: Coming soon</p>
			<Button kind="danger-tertiary" iconDescription="Delete" icon={TrashCan} />
			<Button>Edit</Button>
		</Column>
		<Column sm={4} md={4} lg={4}>
			<StructuredList>
				<StructuredListBody>
					<StructuredListRow>
						<StructuredListCell noWrap>Account</StructuredListCell>
						<StructuredListCell>{data.account_name}</StructuredListCell>
					</StructuredListRow>
					<StructuredListRow>
						<StructuredListCell noWrap>Industry</StructuredListCell>
						<StructuredListCell>{data.industry}</StructuredListCell>
					</StructuredListRow>
					<StructuredListRow>
						<StructuredListCell noWrap>Created</StructuredListCell>
						<StructuredListCell>
							{DateTime.fromISO(data.created_at).toRelativeCalendar()}
						</StructuredListCell>
					</StructuredListRow>
				</StructuredListBody>
			</StructuredList>
		</Column>

		<Column sm={4} md={8} lg={9}>
			<Tile>Recovery Keys and more additional Information is coming soon...</Tile>
		</Column>
	</Row>
</Grid>
