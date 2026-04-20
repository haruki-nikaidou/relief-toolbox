export type PasswordOptions = {
  number: boolean;
  uppercase: boolean;
  symbol: boolean;
  pin: boolean;
};

const CHARSETS = {
  lowercase: "abcdefghijklmnopqrstuvwxyz",
  numbers: "0123456789",
  uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
  symbols: "!@#$%^&*()_+{}[]|:;<>,.?~",
} as const;

export const PASSWORD_SLIDER_MIN = 0;
export const PASSWORD_SLIDER_MAX = 511;
export const PASSWORD_LENGTH_MIN = 6;
export const PASSWORD_LENGTH_MAX = 32;
export const PASSWORD_SLIDER_DEFAULT = 119;

export function mapSliderToPasswordLength(value: number): number {
  return Math.round(
    (value / PASSWORD_SLIDER_MAX) *
      (PASSWORD_LENGTH_MAX - PASSWORD_LENGTH_MIN) +
      PASSWORD_LENGTH_MIN,
  );
}

function getPasswordCharset(options: PasswordOptions): string {
  if (options.pin) {
    return CHARSETS.numbers;
  }

  let charset = CHARSETS.lowercase;
  if (options.number) charset += CHARSETS.numbers;
  if (options.uppercase) charset += CHARSETS.uppercase;
  if (options.symbol) charset += CHARSETS.symbols;

  return charset;
}

export function generatePassword(
  length: number,
  options: PasswordOptions,
): string {
  const charset = getPasswordCharset(options);
  const randomValues = crypto.getRandomValues(new Uint32Array(length));

  return Array.from(randomValues)
    .map((value) => charset[value % charset.length])
    .join("");
}
