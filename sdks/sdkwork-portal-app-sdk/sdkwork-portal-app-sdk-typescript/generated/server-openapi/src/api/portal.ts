import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { PortalPreferencesUpdateRequest } from '../types';


export class PortalPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve current user portal preferences */
  async retrieve(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(appApiPath(`/portal/preferences`));
  }

/** Update current user portal preferences */
  async update(body: PortalPreferencesUpdateRequest): Promise<Record<string, unknown>> {
    return this.client.put<Record<string, unknown>>(appApiPath(`/portal/preferences`), body, undefined, undefined, 'application/json');
  }
}

export class PortalApi {
  private client: HttpClient;
  public readonly preferences: PortalPreferencesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.preferences = new PortalPreferencesApi(client);
  }

}

export function createPortalApi(client: HttpClient): PortalApi {
  return new PortalApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
