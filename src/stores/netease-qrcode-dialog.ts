import { atom } from "jotai";

const atomSteps = atom(["scanning", "confirm", "success"]);

const atomStepIndex = atom<number>(0);

export const atomStepsRead = atom(get => get(atomSteps));

export const atomStepIndexRead = atom(get => get(atomStepIndex));

export const atomStepNext = atom(
  null,
  (get, set) => {
    if(get(atomStepIndex) == 2) return;
    set(atomStepIndex, (get(atomStepIndex) + 1) % 3);
  }
);
