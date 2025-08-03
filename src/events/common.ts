import type { ArgsOf } from 'discordx';
import { Discord, On } from 'discordx';

@Discord()
export class Example {
  @On({ event: 'interactionCreate' })
  memberCheck([interaction]: ArgsOf<'interactionCreate'>): void {
    // Handle interaction creation
    console.log('Interaction created:', interaction.id);
  }
}
