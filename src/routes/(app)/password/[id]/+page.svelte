<script>
	import '@material/web/button/filled-button';
	import '@material/web/button/outlined-button';
	import '@material/web/iconbutton/filled-icon-button';
	import '@material/web/textfield/outlined-text-field';
	import '@material/web/icon/icon';
	import '@material/web/list/list';
	import '@material/web/list/list-item';
	import '@material/web/divider/divider';
	import '@material/web/iconbutton/icon-button';
	import { DateTime } from 'luxon';

	export let data;
</script>

<div class="grid-container">
	<div class="institution container">
		<md-icon-button href="/">
			<md-icon>arrow_back</md-icon>
		</md-icon-button>
		<h3>{data.account.account.institution.name}</h3>
	</div>

	<div class="password container">
		<h1>{data.password}</h1>
		<md-filled-icon-button>
			<md-icon>content_copy</md-icon>
		</md-filled-icon-button>
	</div>

	<div class="data container">
		<md-list>
			<md-list-item>
				Identity: {data.account.account.identity}
			</md-list-item>
			<md-divider />
			<md-list-item>
				Created: {DateTime.fromISO(data.account.account.created_at).toRelativeCalendar()}
			</md-list-item>
			<md-divider />
			<md-list-item>
				Industry: {data.account.industry}
			</md-list-item>
			<md-divider />
			<md-list-item>
				Recovery: {data.account.account.recovery || '/'}
			</md-list-item>
		</md-list>

		<md-outlined-button trailing-icon>
			Edit <md-icon slot="icon">edit</md-icon>
		</md-outlined-button>
		<md-outlined-button trailing-icon>
			Delete <md-icon slot="icon">delete</md-icon>
		</md-outlined-button>
	</div>
</div>

<style>
	.grid-container {
		display: grid;
		grid-template-columns: 3fr 2fr;
		grid-template-rows: auto;
		grid-gap: 16px;
		grid-template-areas:
			'institution data'
			'password data';
		place-items: stretch;
		place-content: stretch;
	}

	.container {
		display: flex;
		flex-flow: column wrap;
		justify-content: flex-start;
		align-items: stretch;
		height: calc(100% - 32px);
		width: calc(100% - 32px);
		padding: 16px;
	}

	.password {
		grid-area: password;
		flex-flow: row nowrap;
	}

	.institution {
		grid-area: institution;
		flex-flow: row wrap;
	}

	.data {
		grid-area: data;
	}
</style>
