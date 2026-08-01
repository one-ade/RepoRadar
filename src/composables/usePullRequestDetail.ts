import type { Ref } from "vue";
import { ref, watch } from "vue";

import {
  getPullRequestDetail,
  type GithubOverview,
  type GithubPullRequest,
  type GithubPullRequestDetail,
  type Project,
} from "../api";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

export function usePullRequestDetail(
  project: Ref<Project | null>,
  overview: Ref<GithubOverview | null>,
  runAction: RunAction,
) {
  const selectedPullRequest = ref<GithubPullRequest | null>(null);
  const pullRequestDetail = ref<GithubPullRequestDetail | null>(null);
  let requestId = 0;

  function clearPullRequest() {
    requestId += 1;
    selectedPullRequest.value = null;
    pullRequestDetail.value = null;
  }

  async function viewPullRequest(pullRequest: GithubPullRequest) {
    if (!project.value) return;
    const path = project.value.path;
    const currentRequest = ++requestId;
    selectedPullRequest.value = pullRequest;
    pullRequestDetail.value = null;
    await runAction(async () => {
      const detail = await getPullRequestDetail(path, pullRequest.number);
      if (currentRequest === requestId) pullRequestDetail.value = detail;
    }, `加载 Pull Request #${pullRequest.number}`);
  }

  watch([project, overview], clearPullRequest);

  return {
    selectedPullRequest,
    pullRequestDetail,
    viewPullRequest,
    clearPullRequest,
  };
}
