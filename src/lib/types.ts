export enum Industry {
	Tech = 'Tech', // -> @
	Games = 'Games', // -> !
	Social = 'Social', // -> #
	Finance = 'Finance', // -> $
	Shopping = 'Shopping', // -> *
	Science = 'Science', // -> ?
	Other = 'Other' // -> &
}

export interface SerializedError {
	status: number;
	message: string;
}
