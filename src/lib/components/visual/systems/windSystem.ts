// =============================================================================
// WIND SYSTEM - Global wind that affects particles and clouds
// =============================================================================

import type { WindState } from '../types';
import { WIND_CONFIG, randomRange, lerp, clamp } from '../constants';

// -----------------------------------------------------------------------------
// Initialization
// -----------------------------------------------------------------------------

export function initWindSystem(): WindState {
  return {
    strength: WIND_CONFIG.BASE_STRENGTH,
    targetStrength: WIND_CONFIG.BASE_STRENGTH,
    gustTimer: randomRange(5000, 15000), // Time until first gust
    isGusting: false,
  };
}

// -----------------------------------------------------------------------------
// Update
// -----------------------------------------------------------------------------

export function updateWindSystem(state: WindState, deltaTime: number): WindState {
  let { strength, targetStrength, gustTimer, isGusting } = state;
  
  // Ease current strength towards target
  strength = lerp(strength, targetStrength, WIND_CONFIG.CHANGE_SPEED * (deltaTime / 16));
  
  // Update gust timer
  gustTimer -= deltaTime;
  
  if (gustTimer <= 0) {
    if (isGusting) {
      // End gust - return to normal
      isGusting = false;
      targetStrength = randomRange(-WIND_CONFIG.BASE_STRENGTH, WIND_CONFIG.BASE_STRENGTH);
      gustTimer = randomRange(8000, 20000); // Time until next gust
    } else {
      // Check for new gust
      if (Math.random() < 0.3) { // 30% chance to gust when timer expires
        isGusting = true;
        // Gust in a random direction
        const gustDirection = Math.random() > 0.5 ? 1 : -1;
        targetStrength = gustDirection * (WIND_CONFIG.BASE_STRENGTH + WIND_CONFIG.GUST_STRENGTH);
        gustTimer = WIND_CONFIG.GUST_DURATION;
      } else {
        // Just change wind direction slightly
        targetStrength = randomRange(-WIND_CONFIG.BASE_STRENGTH * 2, WIND_CONFIG.BASE_STRENGTH * 2);
        gustTimer = randomRange(5000, 12000);
      }
    }
  }
  
  // Clamp strength
  strength = clamp(strength, -WIND_CONFIG.MAX_STRENGTH, WIND_CONFIG.MAX_STRENGTH);
  
  return { strength, targetStrength, gustTimer, isGusting };
}

// -----------------------------------------------------------------------------
// Export bundle
// -----------------------------------------------------------------------------

export const windSystem = {
  init: initWindSystem,
  update: updateWindSystem,
};
