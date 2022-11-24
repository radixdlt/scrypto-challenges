ggplot(df3, aes(
  x = factor(Sterol, level = c("Lanosterol", "Dihydrolanosterol", "Lathosterol", "Desmosterol", "Cholestanol")),
  y = Value,
  fill = factor(Genotype, level = c("Parent", "Affected"))
)) +
  geom_bar(position = "dodge", stat = "summary", fun = "mean", color = "#808080", linewidth = 2) +
  geom_point(
    aes(
      x = factor(Sterol, level = c("Lanosterol", "Dihydrolanosterol", "Lathosterol", "Desmosterol", "Cholestanol")),
      size = 1
    ),
    position = position_jitterdodge(dodge.width = 0.9, jitter.height = 0),
  ) +
  geom_errorbar(stat = "summary", fun.data = "mean_sdl", fun.args = list(mult = 1), position = position_dodge(width = 0.9), width = 0.25, size = 1) +
  scale_fill_manual("Genotype", values = c("Parent" = "#808080", "Affected" = "white")) +
  xlab("") +
  ylab("ng/μg") +
  theme_classic() +
  theme(legend.title = element_blank()) +
  theme(legend.position = c(0.1, 0.9)) +
  theme(axis.text.x = element_text(size = 22, angle = 45, hjust = 1, vjust = 1, color = "black")) +
  theme(axis.text.y = element_text(size = 22, color = "black")) +
  theme(axis.title.y = element_text(size = 22)) +
  theme(axis.line = element_line(colour = "black", linewidth = 1.5)) +
  theme(axis.ticks = element_line(colour = "black", linewidth = 1.5)) +
  theme(legend.text = element_text(size = 22)) +
  guides(size = "none")