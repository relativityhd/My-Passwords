/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function signin(identity: string, password: string, remember: boolean) {
    return invoke()<null>("signin", { identity,password,remember })
}

export function signup(email: string, username: string, password: string, remember: boolean) {
    return invoke()<null>("signup", { email,username,password,remember })
}

export function signout() {
    return invoke()<null>("signout")
}

export function isAuthenticated() {
    return invoke()<boolean>("is_authenticated")
}

export function liveInput(institution: string, identity: string, industry: Industry) {
    return invoke()<string>("live_input", { institution,identity,industry })
}

export function create(institutionName: string, institutionWebsite: string | null, identity: string, recovery: string | null, industry: Industry, bucketid: string | null) {
    return invoke()<string>("create", { institutionName,institutionWebsite,identity,recovery,industry,bucketid })
}

export function get(id: string) {
    return invoke()<SecureAccount>("get", { id })
}

export type Account = { created_at: string; identity: string; recovery: string | null }
export type SecureAccount = { industry: Industry; account: Account }
export type Industry = "Tech" | "Games" | "Social" | "Finance" | "Shopping" | "Science" | "Other"
