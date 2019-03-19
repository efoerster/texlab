import * as https from 'https';
import * as TurndownService from 'turndown';
import { MarkupContent, MarkupKind } from 'vscode-languageserver';

export interface ComponentMetadata {
  caption: string;
  documentation: MarkupContent;
}

interface PackageDescription {
  language?: string;
  text: string;
}

interface Package {
  name: string;
  caption: string;
  descriptions: PackageDescription[];
  errors?: any;
}

const turndownService = new TurndownService();

export async function getComponentMetadata(
  name: string,
): Promise<ComponentMetadata | undefined> {
  return new Promise((resolve, reject) => {
    const request = https.get(
      `https://ctan.org/json/2.0/pkg/${name}`,
      response => response.on('data', data => resolve(parseResponse(data))),
    );

    request.on('error', () => reject());
  });
}

function parseResponse(response: any): ComponentMetadata | undefined {
  const { caption, descriptions, errors } = JSON.parse(response) as Package;
  if (errors) {
    return undefined;
  }

  const description = descriptions.find(x => x.language !== undefined);
  if (description !== undefined) {
    return {
      caption,
      documentation: {
        kind: MarkupKind.Markdown,
        value: turndownService.turndown(description.text),
      },
    };
  }

  return undefined;
}
