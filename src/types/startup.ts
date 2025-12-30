export interface StartupItem {
  id: string;
  name: string;
  description: string | null;
  path: string;
  command: string;
  icon: string | null;
  source: string;
  source_type: "registry" | "folder";
  source_location: string;
  enabled: boolean;
  valid: boolean;
}
