import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { PageInfo } from '../types';


export class PortalPreferencesAdminApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List portal preferences for tenant administration */
  async list(): Promise<Record<string, unknown>> {
    return this.client.get<Record<string, unknown>>(backendApiPath(`/portal/preferences`));
  }
}

export class PortalPreferencesApi {
  private client: HttpClient;
  public readonly admin: PortalPreferencesAdminApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.admin = new PortalPreferencesAdminApi(client);
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
