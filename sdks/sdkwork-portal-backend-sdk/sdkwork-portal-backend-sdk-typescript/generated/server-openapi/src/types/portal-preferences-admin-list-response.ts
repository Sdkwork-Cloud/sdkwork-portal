import type { PageInfo } from './page-info';

export interface PortalPreferencesAdminListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
