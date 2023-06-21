import { render } from "@/tests/utils/base";
import { composeStories } from "@storybook/testing-react";
import * as stories from "picasso-storybook/stories/atoms/chip.stories";

const { ChipTemplate } = composeStories(stories);

test("renders <Chip /> properly", () => {
  const { container } = render(<ChipTemplate />);

  expect(container.getElementsByClassName("MuiChip-root").length).toBe(1);
});
