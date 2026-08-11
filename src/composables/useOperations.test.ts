import { describe, expect, it } from "vitest";

import { useOperations } from "./useOperations";

describe("useOperations", () => {
  it("tracks running, successful, and failed operations", () => {
    const ledger = useOperations();
    const successId = ledger.beginOperation("Fetch");
    const failureId = ledger.beginOperation("Push");

    expect(ledger.activeOperationCount.value).toBe(2);

    ledger.finishOperation(successId, "success");
    ledger.finishOperation(failureId, "failed", new Error("rejected"));

    expect(ledger.activeOperationCount.value).toBe(0);
    expect(ledger.operations.value.find(({ id }) => id === successId)?.state).toBe("success");
    expect(ledger.operations.value.find(({ id }) => id === failureId)).toMatchObject({
      state: "failed",
    });
  });

  it("keeps only the newest 50 records and can clear them", () => {
    const ledger = useOperations();

    for (let index = 0; index < 51; index += 1) {
      ledger.beginOperation(`Operation ${index}`);
    }

    expect(ledger.operations.value).toHaveLength(50);
    expect(ledger.operations.value[0]?.label).toBe("Operation 50");
    expect(ledger.operations.value.at(-1)?.label).toBe("Operation 1");

    ledger.clearOperations();
    expect(ledger.operations.value).toEqual([]);
  });

  it("does not retain failure details in operation records", () => {
    const ledger = useOperations();
    const operationId = ledger.beginOperation("同步仓库");

    ledger.finishOperation(
      operationId,
      "failed",
      new Error("gh --token secret-token D:/private/repo"),
    );

    expect(ledger.operations.value[0]).not.toHaveProperty("error");
  });
});
